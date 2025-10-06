require 'fileutils'
require 'set'
require 'json'

require_relative 'partial_builder'
require_relative 'shellconfig'

class RustBuilder < PartialBuilder
  def initialize
    super

    base_spec_dir = 'spec/rust'
    @base_spec_dir = File.absolute_path(base_spec_dir)

    @spec_dir          = File.join(@base_spec_dir, 'tests')
    @formats_dir       = File.join(@base_spec_dir, 'src/formats')
    @spec_list_file    = File.join(@base_spec_dir, 'tests/spec.rs')
    @formats_list_file = File.join(@base_spec_dir, 'src/formats.rs')

    test_out_dir = File.join(@config['TEST_OUT_DIR'], 'rust')
    @test_out_dir = File.absolute_path(test_out_dir)
  end

  def list_mandatory_files
    []
  end

  def list_disposable_files
    spec_basenames = Dir.glob('test_*.rs', base: @spec_dir)
    format_basenames = Dir.glob('*.rs', base: @formats_dir)

    spec_basenames.map { |fn| File.join(@spec_dir, fn) } +
      format_basenames.map { |fn| File.join(@formats_dir, fn) }
  end

  def create_project(files)
    grouped_files = files.group_by { |fn| file_to_kind(fn) }
    if grouped_files.key?(nil)
      raise "unexpected files that are neither ':spec' or ':format': #{grouped_files[nil].inspect}"
    end

    File.open(@spec_list_file, 'w') { |f|
      f.puts '#![allow(unused_variables)]'
      f.puts '#![allow(unused_assignments)]'
      grouped_files[:spec].each { |fn| f.puts "pub mod #{File.basename(fn, '.rs')};" }
    }
    File.open(@formats_list_file, 'w') { |f|
      f.puts '#![allow(unused_parens)]'
      f.puts '#![allow(dead_code)]'
      grouped_files[:format].each { |fn| f.puts "pub mod #{File.basename(fn, '.rs')};" }
    }
    [@spec_list_file, @formats_list_file]
  end

  def build_project(log_file)
    Dir.chdir(@base_spec_dir) do
      # We don't use `cargo check` here (which would seem like a more logical
      # choice) because unfortunately it doesn't report all build errors, see
      # <https://doc.rust-lang.org/cargo/commands/cargo-check.html#description>:
      #
      # > Some diagnostics and errors are only emitted during code generation,
      # > so they inherently won't be reported with `cargo check`.
      cli = %w[cargo test --no-run --test spec --message-format json]
      run_cargo_build({}, cli, log_file).exitstatus
    end
  end

  def parse_failed_build(log_file)
    list = []

    File.open(log_file, 'r') { |f|
      f.each_line { |line|
        line.chomp!
        # See https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages
        cargo_msg = JSON.parse(line)

        # See <https://doc.rust-lang.org/cargo/reference/external-tools.html#build-finished>:
        # > The "build-finished" message is emitted at the end of the build.
        #
        # > This message can be helpful for tools to know when to stop reading JSON messages.
        break if cargo_msg['reason'] == 'build-finished'

        next unless cargo_msg['reason'] == 'compiler-message'

        # See https://doc.rust-lang.org/rustc/json.html
        rustc_msg = cargo_msg['message']
        next unless rustc_msg['$message_type'] == 'diagnostic'
        next unless rustc_msg['level'] == 'error'

        files =
          select_rustc_msg_culprit_spans(rustc_msg['spans'])
          .map { |span| File.absolute_path(span['file_name'], @base_spec_dir) }

        list.concat(files)
      }
    }

    list
  end

  def file_to_test(path)
    kind = file_to_kind(path)
    basename = File.basename(path, '.rs')
    test_name = kind == :spec ? basename.delete_prefix('test_') : basename
    [kind, underscore_to_ucamelcase(test_name)]
  end

  def run_tests
    Dir.chdir(@base_spec_dir) do
      cli = %w[cargo nextest run --test spec]
      out_log = File.join(@test_out_dir, 'test_run.stdout')
      run_and_tee({}, cli, out_log)

      # See spec/rust/.config/nextest.toml
      src_path = File.join(@base_spec_dir, 'target/nextest/default/junit.xml')
      dest_path = File.join(@test_out_dir, 'report.xml')
      FileUtils.copy_file(src_path, dest_path)
      true
    end
  end

  private

  def select_rustc_msg_culprit_spans(spans)
    primary_spans =
      spans
      .select { |span| span['is_primary'] }
      .map do |span|
        span = span['expansion']['span'] until span['expansion'].nil?
        span
      end
    primary_spans.uniq { |span| span['file_name'] }
  end

  def file_to_kind(path)
    if path_directly_in_dir?(path, @spec_dir)
      :spec
    elsif path_directly_in_dir?(path, @formats_dir)
      :format
    end
  end

  def run_cargo_build(environment, cmd, stdout_file)
    log "running command: #{cmd.inspect}, log: #{stdout_file.inspect}"
    process_status = nil
    FileUtils.mkdir_p(File.dirname(stdout_file))
    File.open(stdout_file, 'w') { |f|
      Open3.popen3(environment, *cmd) { |_stdin, stdout, _stderr, wait_thread|
        while (line = stdout.gets)
          line.chomp!
          line_summary = summarize_cargo_json_line(line)
          puts line_summary unless line_summary.nil?
          f.puts line
        end
        process_status = wait_thread.value
      }
    }
    log "process_status: #{process_status.inspect}"
    process_status
  end

  def summarize_cargo_json_line(line)
    begin
      # See https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages
      cargo_msg = JSON.parse(line)
    rescue JSON::ParserError => e
      warn e.full_message
      return line
    end
    reason = cargo_msg['reason']

    case reason
    when 'compiler-message'
      # See https://doc.rust-lang.org/rustc/json.html
      rustc_msg = cargo_msg['message']

      if rustc_msg['$message_type'] == 'diagnostic'
        files =
          select_rustc_msg_culprit_spans(rustc_msg['spans'])
          .map { |span| "#{span['file_name']}:#{span['line_start']}:#{span['column_start']}" }
        code_appendix = rustc_msg['code'].nil? ? '' : "[#{rustc_msg['code']['code']}]"
        files_prefix =
          if files.empty?
            ''
          else
            "#{files.join(', ')}: "
          end
        "#{files_prefix}#{rustc_msg['level']}#{code_appendix}: #{rustc_msg['message']}"
      else
        line
      end
    when 'compiler-artifact', 'build-script-executed'
      # "[#{reason}] #{cargo_msg['package_id']}"
      nil
    when 'build-finished'
      "[#{reason}] success: #{cargo_msg['success']}"
    else
      line
    end
  end
end

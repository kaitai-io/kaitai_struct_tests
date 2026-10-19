require 'fileutils'
require 'pathname'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class ZigBuilder < PartialBuilder
  def initialize
    super

    base_spec_dir = 'spec/zig'
    @base_spec_dir = File.absolute_path(base_spec_dir)

    @spec_dir       = File.join(@base_spec_dir, 'tests')
    @formats_dir    = File.join(@base_spec_dir, 'formats')
    @spec_list_file = File.join(@base_spec_dir, 'tests.zig')

    test_out_dir = File.join(@config['TEST_OUT_DIR'], 'zig')
    @test_out_dir = File.absolute_path(test_out_dir)

    Kernel.at_exit do
      Dir.glob('*.DISABLED', base: @formats_dir).each do |fn|
        path = File.join(@formats_dir, fn)
        File.rename(path, path.delete_suffix('.DISABLED'))
      end
    end
  end

  def list_mandatory_files
    []
  end

  def list_disposable_files
    spec_basenames = Dir.glob('*_test.zig', base: @spec_dir)
    format_basenames = Dir.glob('*.zig', base: @formats_dir)

    spec_basenames.map { |fn| File.join(@spec_dir, fn) } +
      format_basenames.map { |fn| File.join(@formats_dir, fn) }
  end

  def create_project(files)
    grouped_files = files.group_by { |fn| file_to_kind(fn) }
    if grouped_files.key?(nil)
      raise "unexpected files that are neither ':spec' or ':format': #{grouped_files[nil].inspect}"
    end
    grouped_files[:spec] = [] unless grouped_files.key?(:spec)
    grouped_files[:format] = [] unless grouped_files.key?(:format)

    spec_list_file_dir = File.dirname(@spec_list_file)
    File.open(@spec_list_file, 'w') do |f|
      f.puts 'comptime {'
      grouped_files[:spec].each do |path|
        f.puts "    _ = @import(\"#{Pathname.new(path).relative_path_from(spec_list_file_dir)}\");"
      end
      f.puts '}'
    end

    current_format_basenames = Dir.glob('*.zig', base: @formats_dir)
    current_formats = current_format_basenames.map { |fn| File.join(@formats_dir, fn) }
    included_formats = grouped_files[:format]

    extra_formats = Set.new(current_formats).subtract(included_formats)
    extra_formats.each do |fn|
      File.rename(fn, "#{fn}.DISABLED")
    end

    [@spec_list_file]
  end

  def build_project(log_file)
    Dir.chdir(@base_spec_dir) do
      cli = %w[zig build test --summary all -freference-trace -Dno-bin]
      run_and_tee({}, cli, log_file).exitstatus
    end
  end

  # These messages were found in Zig 0.16.0 source code: https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8
  ZIG_NOTE_MESSAGES_CALLED_HERE = [
    # The following 1 message is from https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/src/Sema.zig#L880
    "called at comptime from here",

    # The following 3 messages are from https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/src/Sema.zig#L2714-L2716
    "generic function instantiated here",
    "called at comptime here",
    "called inline here",

    # The following 1 message is from https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/src/Compilation.zig#L4441
    "file imported here",
  ].freeze
  private_constant :ZIG_NOTE_MESSAGES_CALLED_HERE

  # This error has no source location of its own
  # (https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/src/Zcu.zig#L4949-L4953),
  # but each dependency in the loop has a note with a source location. Notes
  # of such errors are indented by 4 spaces
  # (https://codeberg.org/ziglang/zig/src/commit/24fdd5b7a4c1c8b5deb5b56756b9dbc8e08c86a8/lib/std/zig/ErrorBundle.zig#L318).
  ZIG_DEPENDENCY_LOOP_ERROR = /^error: dependency loop with length \d+$/
  private_constant :ZIG_DEPENDENCY_LOOP_ERROR

  def parse_failed_build(log_file)
    list = []
    in_reference_trace = false
    in_dependency_loop = false

    File.open(log_file, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        clear_in_reference_trace = true
        # Notes of an error without a source location are indented, so
        # anything at the start of a line ends the dependency loop error
        in_dependency_loop = false unless l.start_with?(' ')
        case l
        when ZIG_DEPENDENCY_LOOP_ERROR
          in_dependency_loop = true
        when /^( *)(.+?\.zig):(\d+):(\d+): (error|note): (.*?)$/
          indent = $1
          filename = $2
          # row = $3
          # col = $4
          severity = $5
          msg = $6
          abs_path = File.absolute_path(filename, @base_spec_dir)
          kind = file_to_kind(abs_path)
          case severity
          when 'error'
            if kind == :spec || (kind == :format && msg != "unable to load '#{File.basename(filename)}': FileNotFound")
              list << abs_path
            end
          when 'note'
            if ZIG_NOTE_MESSAGES_CALLED_HERE.include?(msg)
              list << abs_path
            elsif in_dependency_loop && !indent.empty? && (kind == :spec || kind == :format)
              list << abs_path
            end
          end
        when /^ *referenced by:$/
          clear_in_reference_trace = false
          in_reference_trace = true
        when /^ *    (.*?): (.+?\.zig):(\d+):(\d+)$/
          # decl_name = $1
          filename = $2
          # row = $3
          # col = $4
          if in_reference_trace
            clear_in_reference_trace = false
            abs_path = File.absolute_path(filename, @base_spec_dir)
            kind = file_to_kind(abs_path)
            list << abs_path if kind == :spec || kind == :format
          end
        end
        in_reference_trace = false if clear_in_reference_trace
      }
    }

    list
  end

  def file_to_test(path)
    kind = file_to_kind(path)
    basename = File.basename(path, '.zig')
    test_name = kind == :spec ? basename.delete_suffix('_test') : basename
    [kind, underscore_to_ucamelcase(test_name)]
  end

  def run_tests
    Dir.chdir(@base_spec_dir) do
      cli = %w[zig build test --summary all]
      cli.concat(['-p', @test_out_dir])
      out_log = File.join(@test_out_dir, 'test_run.stdout')
      run_and_tee({}, cli, out_log).exitstatus == 0
    end
  end

  private

  def file_to_kind(path)
    if path_directly_in_dir?(path, @spec_dir)
      :spec
    elsif path_directly_in_dir?(path, @formats_dir)
      :format
    end
  end
end

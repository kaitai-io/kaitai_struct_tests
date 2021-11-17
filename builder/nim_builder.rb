require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class NimBuilder < PartialBuilder
  def initialize
    super

    @spec_dir     = File.join('spec', 'nim', 'src')
    @compiled_dir = File.join("#{@config['FORMATS_COMPILED_DIR']}", 'nim')
    @project_file = File.join("#{@compiled_dir}", 'src_list.txt')
    @nim_bin_dir  = File.absolute_path("#{@compiled_dir}/bin")
    @test_out_dir = File.absolute_path("#{@config['TEST_OUT_DIR']}/nim")
  end

  def list_mandatory_files
    []
  end

  def list_disposable_files
    Dir.glob(File.join("#{@spec_dir}", '**/*.nim')) +
    Dir.glob(File.join("#{@compiled_dir}", 'src', '**/*.nim'))
  end

  def create_project(mand_files, disp_files)
    File.open(@project_file, 'w') { |f|
      (mand_files + disp_files).each { |l| f.puts "\"#{l}\"" }
    }
    @project_file
  end

  def build_project(log_file)
    FileUtils.mkdir_p(@nim_bin_dir)
    cli = [
      'nim', 'c',
      '--compileOnly:on',
      '--hints:off',
      "--outdir:#{@nim_bin_dir}"
    ]
    File.open(@project_file).each_line do |f|
      exit_code = run_and_tee({}, "#{cli} #{f}", log_file).exitstatus
      break if exit_code != 0
    end
    return 0
  end

  def parse_failed_build(log_file, disp_files)
    list = Set.new

    File.open(log_file, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        if l =~ /^(.*)\(.*\)Error: .*$/
          filename = $1.gsub('\\', '/')
          list << filename
        end
      }
    }

    list
  end

  def file_to_test(filename)
    # File.basename only forwards with forward slashes, so we normalize for that first
    fn = File.basename(filename.gsub(/\\/, '/'))
    if fn =~ /^t(.*)\.nim$/
      return [:spec, $1.split('_').collect(&:capitalize).join]
    else
      return [:format, fn.gsub(/\.nim$/, '')]
    end
  end

  # Runs tests and generates language-specific test results report.
  # @return [Boolean] true if tests were run successfully, false if no
  #   tests were able to run; note that this does *not* imply that
  #   tests passed or failed - this should be reported in test report file
  #   contents
  def run_tests
    raise NotImplementedError
  end

  def run_and_tee(environment, cmd, stdout_file)
    log "running command: #{cmd.inspect}, log: #{stdout_file.inspect}"
    process_status = nil
    FileUtils.mkdir_p(File.dirname(stdout_file))
    File.open(stdout_file, 'a') { |f|
      Open3.popen2e(environment, *cmd) { |stdin, out, wait_thr|
        while line = out.gets
          puts line
          f.puts line
        end
        process_status = wait_thr.value
      }
    }
    log "process_status: #{process_status.inspect}"
    return process_status
  end
end
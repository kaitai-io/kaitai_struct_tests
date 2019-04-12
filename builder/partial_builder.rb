require 'set'
require 'open3'

require_relative 'shellconfig'

# Common base for all compiled languages, which follow the same
# routine to achieve "partial compilation":
#
# 1. Get a full list of files to compile:
#   * list of "disposable" files, removal of which from compilation
#     might get a partial successful build (compiled/* and specs/*)
#   * list of "mandatory" files (typically, runtime and test unit
#     framework), which cannot be removed at all
# 2. Construct a project file with full list of files
# 3. Build that project file
# 4. If successful, we're done, woo-hoo
# 5. If failed:
#   * Parse project build to get list of files to remove from project
#     in attempt to get it to compile
#   * Build updated project file with these files removed
#   * Retry from step 3
class PartialBuilder
  def initialize
    @config = ShellConfig.new
    log 'initialized'
  end

  def command_line(arg)
    if arg == []
      exit run
    elsif arg == ['--project']
      l1 = list_mandatory_files
      l2 = list_disposable_files
      log "creating project with #{(l1 + l2).size} files"
      fn = create_project(l1, l2)
      log "project file created: #{fn.inspect}"
    else
      puts "Usage: [--project]"
      exit 1
    end
  end

  # 0 = all good
  # 2 = build failed
  # 3 = build good, tests failed
  def run
    return 2 unless partial_build
    return 3 unless run_tests
    0
  end

  def partial_build
    raise "test_out_dir is undefined" unless @test_out_dir
    FileUtils.mkdir_p(@test_out_dir)

    mand_files = Set.new(list_mandatory_files)
    disp_files = Set.new(list_disposable_files)
    orig_size = disp_files.size

    @build_failed_files = "#{@test_out_dir}/build_failed_files.txt"
    @build_failed_tests = "#{@test_out_dir}/build_failed_tests.txt"
    FileUtils.rm_f(@build_failed_files)
    FileUtils.rm_f(@build_failed_tests)

    attempt = 1
    loop {
      log "creating project with #{disp_files.size}/#{orig_size} files"
      fn = create_project(mand_files, disp_files)
      log "project file created: #{fn.inspect}"

      build_log = "#{@test_out_dir}/build-#{attempt}.log"
      log "build attempt #{attempt} (log: #{build_log})"
      if build_project(build_log) == 0
        log attempt == 1 ? "perfect build succeeded" : "success on attempt \##{attempt}, #{disp_files.size}/#{orig_size} files survived"
        return true
      else
        log "build failed"
        if not File.readable?(build_log)
          log "no build log to analyze, won't be able to fix the build by removing bad files"
          log "unable to recover, bailing out :("
          return false
        end

        bad_files = Set.new(parse_failed_build(build_log))
        if bad_files.empty?
          log "build fails, but unable to detect any bad files"
          log "unable to recover, bailing out :("
          return false
        end
        mand_int = mand_files.intersection(bad_files)
        if not mand_int.empty?
          log "errors detected in mandatory files:"
          log mand_int.sort.to_a.join("\n")
          log "unable to recover, bailing out :("
          return false
        end

        # Test if all "bad files" are actually in disposable files
        leftover = bad_files - disp_files

        # Treat "bare" files specially
        leftover_processed = Set.new
        leftover.each { |x|
          if x.respond_to?(:[]) and x[0] == :bare
            # This is indeed a "bare" file - i.e. a file without path
            bare_file = x[1]
            log "removing bare file #{bare_file.inspect}"

            to_delete = Set.new
            disp_files.each { |df|
              to_delete << df if File.basename(df) == bare_file
            }

            if to_delete.empty?
              log "error detected in bare file #{bare_file.inspect}, but unable to file anything like that in disposable files"
              log disp_files.sort.to_a.join("\n")
              log "unable to recover, bailing out :("
              return false
            end

            log "matching disposable files: #{to_delete.to_a.sort.inspect}"

            disp_files -= to_delete
            leftover_processed << x
          end
        }
        leftover -= leftover_processed

        # If there are still any left, report it
        if not leftover.empty?
          log "errors detected in bogus files:"
          log leftover.sort.to_a.join("\n")
          log "unable to recover, bailing out :("
          return false
        end

        # Register bad files we'll exclude
        File.open(@build_failed_files, 'a') { |f|
          bad_files.each { |fn| f.puts fn }
        }

        File.open(@build_failed_tests, 'a') { |f|
          bad_files.each { |fn| f.puts file_to_test(fn).join("\t") }
        }

        disp_files -= bad_files
        attempt += 1
      end
    }
  end

  def list_mandatory_files
    raise NotImplementedError
  end

  def list_disposable_files
    raise NotImplementedError
  end

  # Creates a project file, given a list of disposable and mandatory
  # files to include in it.
  # @return [String] project file name created
  def create_project(mand_files, disp_files)
    raise NotImplementedError
  end

  # Runs a build of a project, relying on previously created project
  # file.
  # @param log_file [String] path to build's log file that is expected
  #   to be created during this build operation.
  # @return [Numeric] 0 if project was built successfully, non-zero if
  #   the build has failed
  def build_project(log_file)
    raise NotImplementedError
  end

  # Parses log file for a failed build and returns a list of files
  # that had compilation errors.
  # @param log_file [String] path to build's log file
  # @return [Array<String>] list of files that have compilation
  #   errors, as reported in the build log file
  def parse_failed_build(log_file)
    raise NotImplementedError
  end

  # Converts given file name to [spec or format, test name]. "Spec of
  # format" must be a :spec or :format symbol. Test name must be in
  # UpperCamelCase.
  def file_to_test(filename)
    raise NotImplementedError
  end

  # Runs tests and generates language-specific test results report.
  # @return [Boolean] true if tests were run successfully, false if no
  #   tests were able to run; note that this does *not* imply that
  #   tests passed or failed - this should be reported in test report file
  #   contents
  def run_tests
    raise NotImplementedError
  end

  # ====================================================================

  # Logs message to stdout, prepending a visible token that can be
  # used for grepping these logs afterwards. Flushes stdout/stderr
  # before and after that to sync/straighten CI console
  # implementations for better readability.
  # @param msg message to log
  def log(msg)
    $stdout.flush
    $stderr.flush
    puts "#### #{self.class}: #{msg}"
    $stdout.flush
    $stderr.flush
  end

  def run_and_tee(environment, cmd, stdout_file)
    log "running command: #{cmd.inspect}, log: #{stdout_file.inspect}"
    process_status = nil
    FileUtils.mkdir_p(File.dirname(stdout_file))
    File.open(stdout_file, 'w') { |f|
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

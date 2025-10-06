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
    @max_attempts = nil
    log 'initialized'
  end

  def command_line(arg)
    if arg == []
      exit run
    elsif arg == ['--project']
      files1 = list_mandatory_files
      files2 = list_disposable_files
      files_all = files1 + files2
      log "creating project with #{files_all.size} files"
      fns = create_project(files_all)
      log "project files created: #{fns.inspect}"
    elsif arg == ['--once']
      @max_attempts = 1
      exit run
    else
      puts "Usage: [--project|--once]"
      exit 1
    end
  end

  # 0 = all good
  # 2 = build failed
  # 3 = build good, tests failed
  def run
    return 2 unless partial_build

    log "running tests"
    t1 = Time.now
    tests_result = run_tests
    t2 = Time.now
    log "running tests: elapsed: #{t2 - t1}s"

    return 3 unless tests_result
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

    # In Ruby 2.6, this could be just `(1..).each { ... }` without the need for
    # `Float::INFINITY`, but at the time of writing, some of our targets are
    # still using older versions of Ruby, such as Ruby 1.9.
    (1..Float::INFINITY).each { |attempt|
      attempt_str = @max_attempts ? "#{attempt}/#{@max_attempts}" : attempt

      log "creating project with #{disp_files.size}/#{orig_size} files"
      files1 = mand_files.to_a
      files2 = disp_files.to_a
      files_all = files1 + files2
      fns = create_project(files_all)
      log "project files created: #{fns.inspect}"

      build_log = "#{@test_out_dir}/build-#{attempt}.log"
      log "build attempt #{attempt_str} (log: #{build_log})"
      t1 = Time.now
      result = build_project(build_log)
      t2 = Time.now
      log "build attempt #{attempt_str}: elapsed: #{(t2 - t1).to_i}s"
      if result == 0
        log attempt == 1 ? "perfect build succeeded" : "success on build attempt #{attempt_str}, #{disp_files.size}/#{orig_size} files survived"
        return true
      else
        log "build failed"
        bad_files = adjust_files_for_failed_build(build_log, mand_files, disp_files)

        if bad_files == false
          log "unable to recover, bailing out :("
          return false
        end

        register_bad_files(bad_files)

        if @max_attempts and attempt >= @max_attempts
          log "maximum number of build attempts reached, bailing out"
          return false
        end
      end
    }
  end

  # Adjusts list of disposable files by disposing of "bad" files, as
  # per a log file of failed build.
  # @param build_log [String] path to build log file
  # @param mand_files [Set<String>] set of mandatory files
  # @param disp_files [Set<String>] set of disposable files (to be modified!)
  # @return [Set<String>, FalseClass] set of bad files if disposal of bad
  #   files was succesful (and we can do another build attempt), false
  #   if we've encountered some unrecoverrable error and thus it's
  #   pointless to retry.
  def adjust_files_for_failed_build(build_log, mand_files, disp_files)
    if not File.readable?(build_log)
      log "no build log to analyze, won't be able to fix the build by removing bad files"
      return false
    end

    orig_disp_size = disp_files.size

    bad_files = Set.new(parse_failed_build(build_log))
    if bad_files.empty?
      log "build fails, but unable to detect any bad files"
      return false
    end
    mand_int = mand_files.intersection(bad_files)
    if not mand_int.empty?
      log "errors detected in mandatory files:"
      log mand_int
      return false
    end

    # Test if all "bad files" are actually in disposable files
    leftover = bad_files - disp_files

    # Treat "bare" files specially
    leftover_processed = Set.new
    leftover.each { |x|
      if is_bare?(x)
        # This is indeed a "bare" file - i.e. a file without path
        bare_file = x[1]
        log "removing bare file #{bare_file.inspect}"

        to_delete = Set.new
        disp_files.each { |df|
          to_delete << df if File.basename(df.gsub(/\\/, '/')) == bare_file
        }

        if to_delete.empty?
          log "error detected in bare file #{bare_file.inspect}, but unable to file anything like that in disposable files"
          log disp_files
          return false
        end

        log "matching disposable files:"
        log to_delete

        disp_files.subtract(to_delete)
        leftover_processed << x
      end
    }
    leftover.subtract(leftover_processed)

    # If there are still any left, report it
    if not leftover.empty?
      log "errors detected in bogus files:"
      log leftover
      return false
    end

    disp_files.subtract(bad_files)

    # Sanity check: if size of disp_files haven't diminished, we have
    # not changed anything => it will result in endless loop.
    unless disp_files.size < orig_disp_size
      log "sanity check failed: modified list of disposable files is the same as original, nothing is removed"
      return false
    end

    return bad_files
  end

  # Register bad files we'll exclude from the next build: writes two
  # log files "build_failes_files.txt" with a plain list of rejected
  # files and "build_failed_tests" with two tab-separated columns:
  # type of file and name of relevant test.
  # @param bad_files [Set<String>] set of "bad" (rejected) files
  def register_bad_files(bad_files)
    File.open(@build_failed_files, 'a') { |f|
      bad_files.each { |fn| f.puts fn }
    }

    File.open(@build_failed_tests, 'a') { |f|
      bad_files.each { |fn| f.puts file_to_test(fn).join("\t") }
    }
  end

  # @return [Array<String>]
  def list_mandatory_files
    raise NotImplementedError
  end

  # @return [Array<String>]
  def list_disposable_files
    raise NotImplementedError
  end

  # Creates usually one project file (or more, depending on the needs
  # of the language), given a list of source files to include in it.
  # @param files [Array<String>] collection of source files
  # @return [Array<String>] project files created
  def create_project(files)
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

    if msg.is_a?(Set)
      msg.to_a.sort.each { |element|
        puts "#### #{self.class}: - #{element.inspect}"
      }
    else
      puts "#### #{self.class}: #{msg}"
    end

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

  def is_bare?(x)
    x.respond_to?(:[]) and x[0] == :bare
  end

  def sort_list_with_bare(list)
    list.sort { |a, b|
      aa = is_bare?(a) ? a[1] : a
      bb = is_bare?(b) ? b[1] : b
      aa <=> bb
    }
  end

  protected

  # Returns +true+ if +path+ is directly contained in the +dir_abs_path+
  # directory, and +false+ otherwise.
  #
  # The +dir_abs_path+ argument is expected to be normalized as if returned by
  # the +File.absolute_path+ method, i.e. directory parts in path separated only
  # by +File::SEPARATOR+ (forward slash +/+ on all platforms) and never
  # +File::ALT_SEPARATOR+ (backslash +\+ on Windows), multiple consecutive
  # slashes collapsed into one, no useless dots (+./+ or +../+ parts), no
  # trailing slash, etc.
  # @param path [String] relative (to the current directory) or absolute path
  # @param dir_abs_path [String] normalized absolute path of the directory
  def path_directly_in_dir?(path, dir_abs_path)
    abs_path = File.absolute_path(path)
    dir_abs_path += File::SEPARATOR
    return false unless abs_path.start_with?(dir_abs_path)

    path_rel_to_dir = abs_path.delete_prefix(dir_abs_path)
    path_basename = File.basename(abs_path)
    path_rel_to_dir == path_basename
  end

  def underscore_to_ucamelcase(s)
    s.split(/_/).map { |x| x.capitalize }.join
  end
end

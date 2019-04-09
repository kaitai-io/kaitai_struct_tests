require 'set'
require 'open3'

require_relative 'shellconfig'

##
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
      run
    elsif arg == ['--project']
      create_project(list_mandatory_files, list_disposable_files)
    else
      puts "Usage: [--project]"
      exit 1
    end
  end

  def run
    raise "test_out_dir is undefined" unless @test_out_dir

    mand_files = Set.new(list_mandatory_files)
    disp_files = Set.new(list_disposable_files)

    attempt = 1
    loop {
      log "create project with #{disp_files.size} files"
      create_project(mand_files, disp_files)

      build_log = "#{@test_out_dir}/build-#{attempt}.log"
      log "build attempt #{attempt} (log: #{build_log})"
      if build_project(build_log) == 0
        log "success"
        return true
      else
        log "build failed"
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
        if not leftover.empty?
          log "errors detected in bogus files:"
          log leftover.sort.to_a.join("\n")
          log "unable to recover, bailing out :("
          return false
        end

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

  def create_project(disp_files, mand_files)
    raise NotImplementedError
  end

  def build_project(log_file)
    raise NotImplementedError
  end

  def parse_failed_build(log_file)
    raise NotImplementedError
  end

  # ====================================================================

  def log(msg)
    puts "#### #{self.class}: #{msg}"
  end

  def run_and_tee(environment, cmd, stdout_file)
    process_status = nil
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

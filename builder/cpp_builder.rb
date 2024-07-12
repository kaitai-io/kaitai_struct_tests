require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'
require_relative '../aggregate/boost_test_parser'

class CppBuilder < PartialBuilder
  attr_accessor :mode

  def initialize(src_dir, cpp_spec_dir, cpp_test_out_dir)
    super()

    @src_dir = src_dir
    @cpp_spec_dir = cpp_spec_dir
    @test_out_dir = cpp_test_out_dir

    @obj_dir = "#{@src_dir}/bin"
    @disposable_cmake = "#{@obj_dir}/disposable.cmake"
    @abs_cpp_test_out_dir = File.absolute_path(@test_out_dir)
    @test_dir = Dir.pwd

    @mode = :make_posix
    @mode = :msbuild_windows if ENV['APPVEYOR']

    detect_tools
  end

  def detect_tools
    puts "Detecting cmake..."
    cmake_version_out = `cmake --version`
    if cmake_version_out =~ /cmake version (\d+)\.(\d+)\.(\d+)/
      @cmake_version = [$1.to_i, $2.to_i, $3.to_i]
      puts "cmake #{@cmake_version.inspect}"
    else
      puts cmake_version_out
      raise "Unknown cmake version"
    end

    if @mode == :make_posix
      puts "Detecting gmake..."
      @make_cmd = nil
      begin
        make_version_out = `gmake --version`
        @make_cmd = 'gmake'
      rescue Errno::ENOENT
        make_version_out = `make --version`
        @make_cmd = 'make'
      end

      if make_version_out =~ /GNU Make (\d+)\.(\d+)/
        @make_version = [$1.to_i, $2.to_i]
        puts "GNU Make #{@make_version.inspect} as #{@make_cmd}"
      else
        puts make_version_out
        raise "Unknown make version"
      end
    end
  end

  def list_mandatory_files
    []
  end

  def list_disposable_files
    list = Dir.glob("#{@cpp_spec_dir}/**/*.cpp") + Dir.glob("#{@src_dir}/*.cpp")
    list.map { |x|
      r = File.absolute_path(x)

      # On Windows, filesystem is case insensitive, but our Set
      # implementation is not. So we proactively normalize everything we
      # can to lower case.
      if @mode == :msbuild_windows
        r.downcase!
      end

      r
    }
  end

  def create_project(mand_files, disp_files)
    FileUtils.mkdir_p(@obj_dir)
    File.open(@disposable_cmake, 'w') { |f|
      f.puts("set(DISPOSABLE_SOURCES")
      (mand_files + disp_files).each { |l| f.puts(l) }
      f.puts(")")
    }
    @disposable_cmake
  end

  def build_project(log_file)
    log "doing pre-build: cmake"
    e1 = run_cmake("cmake_#{log_file}")
    return e1 if e1 != 0

    log "doing build"
    e2 = run_build(log_file)
    return e2
  end

  def run_cmake(log_file)
    orig_dir = Dir.pwd
    Dir.chdir(@obj_dir)

    cmake_cli = [
      "cmake",
      "-DCMAKE_BUILD_TYPE=Debug",
      "-DINC_PATH=#{File.absolute_path(@disposable_cmake)}",
      "-DKS_PATH=#{File.absolute_path(@src_dir)}",
    ]

    # Building on Appveyor/Windows requires extra argument to CMake
    if ENV['APPVEYOR']
      cmake_cli << "-DCMAKE_TOOLCHAIN_FILE=c:/tools/vcpkg/scripts/buildsystems/vcpkg.cmake"

      arch = ENV['ARCH'] || 'x64'
      cmake_cli << "-DCMAKE_GENERATOR_PLATFORM=#{arch}"
    end

    cmake_cli << @cpp_spec_dir

    r = run_and_tee({"LC_ALL" => "en_US.UTF-8"}, cmake_cli, log_file).exitstatus
    Dir.chdir(orig_dir)
    r
  end

  def run_build(log_file)
    abs_log_file = File.absolute_path(log_file)

    orig_dir = Dir.pwd
    Dir.chdir(@obj_dir)

    case @mode
    when :make_posix
      cmd = ["cmake", "--build", ".", "--parallel", "8", "--verbose", "--", "-k"]
      cmd << "--output-sync=target" if @make_version[0] >= 4

      r = run_and_tee(
        {"LC_ALL" => "en_US.UTF-8"},
        cmd,
        abs_log_file
      )
    when :msbuild_windows
      r = run_and_tee(
        {},
        ["msbuild", "KS_TEST_CPP_STL.sln"], # -fl -flp:logfile=#{@abs_cpp_test_out_dir}\\msbuild.log"
        abs_log_file
      )
    else
      raise "Unknown mode=#{@mode}"
    end

    Dir.chdir(orig_dir)
    r
  end

  def parse_failed_build(log_file)
    list = []

    orig_cpp_filename = nil
    orig_cpp_filename_line_no = nil
    parse_mode = :normal

    block_included_from_continued = false

    case @mode
    when :make_posix
      File.open(log_file, 'r') { |f|
        f.each_line.with_index(1) { |l, line_no|
          l.chomp!
          clear_block_included_from_continued = true

          case l
          when /^(?:(In file included) |\s+)from (.+?):(?:\d+)(?::\d+)?([:,])$/
            is_block_start = $1 == 'In file included'
            raise "line #{line_no}: continuation line of block 'In file included from ...' found, but not expected" if !is_block_start && !block_included_from_continued
            orig_cpp_filename = $2
            block_included_from_continued = ($3 == ',')
            clear_block_included_from_continued = false
          when /^(.+?):(\d+):(\d+): (?:fatal )?error: (.*)$/
            filename = $1
            #row = $2
            #col = $3
            #msg = $4

            # .h files are not members of disposable_files per se,
            # they are always included from some other .cpp
            # files. Thus, we report error against original .cpp file,
            # which we've memorized previously.

            if !filename.end_with?('.cpp')
              if !filename.end_with?('.h')
                log "line #{line_no}: found error in #{filename.inspect} file, which has neither .cpp nor .h extension (this is unusual)"
              end
              raise "line #{line_no}: found error in #{filename.inspect} file, but no original .cpp file reference found before" if orig_cpp_filename.nil?
              filename = orig_cpp_filename
            end

            list << filename
          # Linux ld, variant 1
          when /^\/usr\/bin\/ld: ([^:]+?):(\d+): undefined reference/
            filename = $1
            #row = $2
            list << filename
          # Linux ld, variant 2
          when /^([^:]+?):(\d+): undefined reference/
            filename = $1
            #row = $2
            list << filename
          # Mac OS X ld
          when /, referenced from:$/
            parse_mode = :osx_ld
          when /^\s*(.*?) in (.*?\.cpp)\.o$/
            if parse_mode == :osx_ld
              #method = $1
              filename = $2
              list << [:bare, filename]
            end
          end

          if block_included_from_continued && clear_block_included_from_continued
            log "line #{line_no}: implicitly closing the active 'In file included from ...' block (this shouldn't be needed in a valid log file)"
            block_included_from_continued = false
          end
        }
      }
    when :msbuild_windows
      File.open(log_file, 'r') { |f|
        f.each_line.with_index(1) { |l, line_no|
          l.chomp!
          clear_orig_cpp_filename = true
          # c:\projects\ci-targets\tests\compiled\cpp_stl_98\enum_to_i_class_border_2.h(18): error C2061: syntax error: identifier 'enum_to_i_class_border_1_t' [C:\projects\ci-targets\tests\compiled\cpp_stl_98\bin\ks_tests.vcxproj]
          # C:\projects\ci-targets\tests\spec\cpp_stl_98\test_expr_calc_array_ops.cpp(4): fatal error C1083: Cannot open include file: 'expr_calc_array_ops.h': No such file or directory [C:\projects\ci-targets\tests\compiled\cpp_stl_98\bin\ks_tests.vcxproj]
          case l
          when /^\s+([a-z0-9_]+\.cpp)$/
            clear_orig_cpp_filename = false
            orig_cpp_filename = $1
            orig_cpp_filename_line_no = line_no
          when /^(\S+?)\((\d+)\): warning (.*?): (.*)$/
            clear_orig_cpp_filename = false
          when /^(\S+?)\((\d+)\): (:?fatal )?error (.*?): (.*)$/
            clear_orig_cpp_filename = false
            filename = $1
            #row = $2
            #code = $3
            #msg = $4

            # MSBuild is really weird and sometimes uses paths for
            # same files with different upper/lower cases
            filename.downcase!

            # Also, our original globbing used forward slashes, so
            # convert backslashes to forward slashes
            filename.gsub!(/\\/, '/')

            # .h files are not members of disposable_files per se,
            # they are always included from some other .cpp
            # files. MSBuild logs print the original .cpp filename
            # indented with 2 spaces just before any error
            # occurs, so we just use it.
            if filename =~ /\.h$/
              log "line #{line_no}: error in #{filename.inspect}, "\
                "orig_cpp_filename is #{orig_cpp_filename.inspect} from line #{orig_cpp_filename_line_no.inspect}"
              raise "Found error in #{filename.inspect} file, but no original .cpp file reference found before" if orig_cpp_filename.nil?
              filename = [:bare, orig_cpp_filename]
            end
            list << filename
          when /^(\S+?)\.obj : error LNK2019:/
            filename = "#{$1}.cpp"
            list << [:bare, filename]
          end

          if !orig_cpp_filename.nil? && clear_orig_cpp_filename
            log "line #{line_no}: clearing orig_cpp_filename #{orig_cpp_filename.inspect}"\
              " from line #{orig_cpp_filename_line_no.inspect}"
            orig_cpp_filename = nil
            orig_cpp_filename_line_no = nil
          end
        }
      }
    else
      raise "Unknown mode=#{@mode}"
    end

    list
  end

  def file_to_test(filename)
    # Treat bare files as all others
    if is_bare?(filename)
      filename = filename[1]
    end

    # File.basename only forwards with forward slashes, and we might
    # get some backslashes on Windows systems, so we normalize for
    # that first.
    fn = File.basename(filename.gsub(/\\/, '/'))
    if fn =~ /^test_(.*)\.(cpp|h)$/
      [:spec, luc_to_ucc($1)]
    elsif fn =~ /^(.*)\.(cpp|h)$/
      [:format, luc_to_ucc($1)]
    else
      [:unknown, fn]
    end
  end

  def run_tests
    attempt = 1
    excluded_tests = []

    loop {
      attempt_str = @max_attempts ? "#{attempt}/#{@max_attempts}" : attempt

      xml_log = "#{@abs_cpp_test_out_dir}/results-#{attempt}.xml"
      log "run attempt #{attempt_str} (log: #{xml_log})"

      # Choose test executable
      case @mode
      when :msbuild_windows
        # On Windows/MSVC, executable will be nested in Debug/
        ks_tests_bin = "#{@obj_dir}/Debug/ks_tests"
      when :make_posix
        # On POSIX systems, it will be directly in obj dir
        ks_tests_bin = "#{@obj_dir}/ks_tests"
      else
        raise "Unknown mode=#{@mode}"
      end

      tests_cli = [
        ks_tests_bin,
        '--log_format=XML',
        "--log_sink=#{xml_log}",
        '--log_level=all',
        '--report_level=detailed',
      ]
      excluded_tests.each { |test| tests_cli << "--run_test=!#{test}" }

      # Actually run the tests
      Dir.chdir(@test_dir) do
        run_and_tee({}, tests_cli, "#{@abs_cpp_test_out_dir}/test_run-#{attempt}.stdout")
      end

      parser = BoostTestParser.new(xml_log)
      aborted_tests = parser.aborted_tests
      File.write("#{@abs_cpp_test_out_dir}/aborted_tests-#{attempt}.txt", aborted_tests.map { |test| "#{test}\n" }.join)
      if aborted_tests.empty?
        log "success on run attempt #{attempt_str} (#{excluded_tests.size} tests excluded)"
        return true
      end
      log "run attempt #{attempt_str} aborted (#{aborted_tests.size} tests aborted)"
      aborted_tests.each { |test| excluded_tests << test }

      attempt += 1

      if @max_attempts and attempt >= @max_attempts
        log "maximum number of run attempts reached, bailing out"
        return false
      end
    }
  end

  private
  # Converts `lower_underscore_case` to `UpperCamelCase`
  def luc_to_ucc(s)
    s.split(/_/).map { |word| word.capitalize }.join
  end
end

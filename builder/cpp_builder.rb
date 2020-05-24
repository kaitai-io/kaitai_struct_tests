require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

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
      r = run_and_tee(
        {"LC_ALL" => "en_US.UTF-8"},
        ["make", "-j8", "-k"],
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

  def parse_failed_build(log_file, disp_files)
    list = Set.new

    orig_cpp_filename = nil
    parse_mode = :normal

    case @mode
    when :make_posix
      File.open(log_file, 'r') { |f|
        f.each_line { |l|
          l.chomp!
          case l
          when /^In file included from (.+?):(\d+)(:\d+)?:$/
            orig_cpp_filename = $1
          when /^(.+?):(\d+):(\d+): (?:fatal )?error: (.*)$/
            filename = $1
            #row = $2
            #col = $3
            #msg = $4

            # .h files are not members of disposable_files per se,
            # they are always included from some other .cpp
            # files. Thus, we report error against original .cpp file,
            # which we've memorized previously.
            if filename =~ /\.h$/
              raise "Found error in #{filename.inspect} file, but no original .cpp file reference found before" if orig_cpp_filename.nil?
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
        }
      }
    when :msbuild_windows
      File.open(log_file, 'r') { |f|
        f.each_line { |l|
          l.chomp!
          # c:\projects\ci-targets\tests\compiled\cpp_stl_98\enum_to_i_class_border_2.h(18): error C2061: syntax error: identifier 'enum_to_i_class_border_1_t' [C:\projects\ci-targets\tests\compiled\cpp_stl_98\bin\ks_tests.vcxproj]
          # C:\projects\ci-targets\tests\spec\cpp_stl_98\test_expr_calc_array_ops.cpp(4): fatal error C1083: Cannot open include file: 'expr_calc_array_ops.h': No such file or directory [C:\projects\ci-targets\tests\compiled\cpp_stl_98\bin\ks_tests.vcxproj]
          case l
          when /^\s+([a-z0-9_]+\.cpp)$/
            orig_cpp_filename = disp_files.find { |path| path.end_with?($1) }
          when /^\s*(.*?)\((\d+)\): (:?fatal )?error (.*?): (.*)$/
            filename = $1
            #row = $2
            #code = $3
            #msg = $4

            # MSBuild is really weird and sometimes uses paths for
            # same files with different upper/lower cases
            filename = filename.downcase

            # Also, our original globbing used forward slashes, so
            # convert backslashes to forward slashes
            filename.gsub!(/\\/, '/')

            # .h files are not members of disposable_files per se,
            # they are always included from some other .cpp
            # files. Msbuild logs print the original .cpp filename
            # indented with 2 spaces just before any error
            # occurs, so we just use it.
            if filename =~ /(.*)\.h$/
              raise "Found error in #{filename.inspect} file, but no original .cpp file reference found before" if orig_cpp_filename.nil?
              filename = orig_cpp_filename
            end
            list << filename
          when /^\s*(.*?)\.obj : error LNK2019:/
            filename = "#{$1}.cpp"
            list << [:bare, filename]
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
    if filename.respond_to?(:[]) and filename[0] == :bare
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
    xml_log = "#{@abs_cpp_test_out_dir}/results.xml"

    # Work around boost v1.62 bug: https://svn.boost.org/trac10/ticket/12507
    # --log_sink is broken in boost v1.62, using workaround, as per
    # https://stackoverflow.com/a/39999085/487064
    #
    # However, Travis has boost v1.54, which has problems with it, so we
    # won't use the workaround on anything except exactly v1.62

    boost_log_option = "--log_sink=#{xml_log}"

    begin
      if File.read('/usr/include/boost/version.hpp') =~ /BOOST_VERSION 106200/
        # Boost v1.62 detected, enabling workaround
        boost_log_option = "--logger=JUNIT,test_suite,#{xml_log}"
      end
    rescue Errno::ENOENT => e
      # ignore
    end

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
      boost_log_option,
      '--log_level=all',
      '--report_level=detailed',
    ]

    # Actually run the tests
    Dir.chdir(@test_dir)
    run_and_tee({}, tests_cli, "#{@abs_cpp_test_out_dir}/test_run.stdout")

    File.exists?(xml_log)
  end

  private
  # Converts `lower_underscore_case` to `UpperCamelCase`
  def luc_to_ucc(s)
    s.split(/_/).map { |word| word.capitalize }.join
  end
end

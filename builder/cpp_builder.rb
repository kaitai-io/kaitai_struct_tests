require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class CppBuilder < PartialBuilder
  def initialize(src_dir, cpp_spec_dir, cpp_test_out_dir)
    @config = ShellConfig.new('config')

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
    list.map { |x| File.absolute_path(x) }
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

    if File.exists?('Makefile')
      r = run_and_tee(
        {"LC_ALL" => "en_US.UTF-8"},
        ["make", "-j8", "-k"],
        abs_log_file
      )
    elsif File.exists?('KS_TEST_CPP_STL.sln')
      r = run_and_tee(
        {},
        ["msbuild", "KS_TEST_CPP_STL.sln"], # -fl -flp:logfile=#{@abs_cpp_test_out_dir}\\msbuild.log"
        abs_log_file
      )
    else
      raise "No build makefile/project file found, unable to continue."
    end

    Dir.chdir(orig_dir)
    r
  end

  def parse_failed_build(log_file)
    list = Set.new

    case @mode
    when :make_posix
      File.open(log_file, 'r') { |f|
        f.each_line { |l|
          l.chomp!
          if l =~ /^(.*?):(\d+):(\d+): error: (.*)$/
            filename = $1
            #row = $2
            #col = $3
            #msg = $4
            list << filename
          elsif l =~ /^(.*?):(\d+): undefined reference/
            filename = $1
            #row = $2
            list << filename
          end
        }
      }
    else
      raise "Unknown mode={@mode}"
    end

    list
  end

  def file_to_test(filename)
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
    if File.exists?("#{@obj_dir}/KS_TEST_CPP_STL.sln")
      # On Windows/MSVC, executable will be nested in Debug/
      ks_tests_bin = "#{@obj_dir}/Debug/ks_tests"
    else
      # On POSIX systems, it will be directly in obj dir
      ks_tests_bin = "#{@obj_dir}/ks_tests"
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

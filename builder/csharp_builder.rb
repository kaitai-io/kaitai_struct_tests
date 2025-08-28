require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class CSharpBuilder < PartialBuilder
  def initialize
    super

    @target_net = :netframework
    @target_net = :netstd if ENV["KAITAI_NET"] == "netstd"

    @compiled_dir = 'compiled/csharp'
    @spec_dir = 'spec/csharp/kaitai_struct_csharp_tests'

    case @target_net
    when :netframework
      @project_file = "spec/csharp/kaitai_struct_csharp_tests_netframework/kaitai_struct_csharp_tests_netframework.csproj"
      @out_dll = "spec/csharp/kaitai_struct_csharp_tests_netframework/bin/Debug/kaitai_struct_csharp_tests.dll"
      check_msbuild
    when :netstd
      @project_file = "spec/csharp/kaitai_struct_csharp_tests_netstd/kaitai_struct_csharp_tests_netstd.csproj"
      @out_dll = "spec/csharp/kaitai_struct_csharp_tests_netstd/bin/Debug/net6.0/kaitai_struct_csharp_tests_netstd.dll"
      check_dotnet
    else
      raise "Unknown target .NET: #{@target_net}"
    end

    @project_template = @project_file + ".in"

    @test_out_dir = "#{@config['TEST_OUT_DIR']}/csharp"
  end

  def check_dotnet
    if system("dotnet build --version")
      @msbuild = 'dotnet'
      @msbuild_args = ['build']
    else
      raise 'Unable to find `dotnet build`, bailing out'
    end
  end

  def check_msbuild
    @msbuild_args = []

    # msbuild
    if system("msbuild /version")
      @msbuild = 'msbuild'
    elsif system("xbuild /version")
      @msbuild = 'xbuild'
    else
      raise 'Unable to find msbuild/xbuild, bailing out'
    end

    # If we're running in AppVeyor, add extra logger args
    if ENV['APPVEYOR']
      @msbuild_args << '/logger:C:\Program Files\AppVeyor\BuildAgent\Appveyor.MSBuildLogger.dll'
    end

    # If mono is available, use it
    @is_mono = system("mono --version") ? true : false
  end

  def list_mandatory_files
    paths = [
      "#{@spec_dir}/CommonSpec.cs",
    ]
    paths << "#{@spec_dir}/Properties/AssemblyInfo.cs" if @target_net == :netframework

    convert_slashes(paths.map { |fn| File.absolute_path(fn) })
  end

  def list_disposable_files
    rel_files = Dir.glob("#{@spec_dir}/tests/**/*.cs") + Dir.glob("#{@compiled_dir}/**/*.cs")
    abs_files = rel_files.map { |fn| File.absolute_path(fn) }
    convert_slashes(abs_files)
  end

  def create_project(files)
    tmpl = File.read(@project_template)
    files_xml = files.map { |x| "    <Compile Include=\"#{x}\" />" }.join("\n")
    project = tmpl.gsub(/%%%FILES%%%/, files_xml)
    File.write(@project_file, project)
    [@project_file]
  end

  def build_project(log_file)
    cli = [@msbuild] + @msbuild_args + [@project_file]
    run_and_tee({}, cli, log_file).exitstatus
  end

  def parse_failed_build(log_file)
    list = Set.new

    File.open(log_file, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        if l =~ /^\s*(.*?)\((\d+),(\d+)\): error (.*?): (.*)$/
          filename = $1
          #row = $2
          #col = $3
          #code = $4
          #msg = $5

          if filename =~ /^tests[\\\/]([^\/].*)$/
            # Not an absolute path, this happens when we hit problems
            # originating in spec files - treat these as bare files
            list << [:bare, $1]
          else
            list << filename
          end
        end
      }
    }

    convert_slashes(list)
  end

  def file_to_test(filename)
    fn = if is_bare?(filename)
      # If bare name, then it's exactly what we're looking for
      filename[1]
    else
      # File.basename only forwards with forward slashes, so we normalize for that first
      File.basename(filename.gsub(/\\/, '/'))
    end
    if fn =~ /^Spec(.*)\.cs$/
      return [:spec, $1]
    else
      return [:format, fn.gsub(/\.cs$/, '')]
    end
  end

  # Detect where the runner is and blow up if it's not found
  def detect_runner
    candidates = [
      ENV['HOME'] + "/.nuget/packages/nunit.consolerunner/3.19.0/tools/nunit3-console.exe",
      "spec/csharp/packages/NUnit.ConsoleRunner.3.19.0/tools/nunit3-console.exe",
    ]

    candidates.each { |c|
      return c if File.exist?(c)
    }

    raise "Unable to find NUnit.ConsoleRunner.3.19.0 exe file anywhere, tried: #{candidates.inspect}"
  end

  def run_tests
    FileUtils.mkdir_p(@test_out_dir)
    xml_log = nil

    if @msbuild == 'dotnet'
      xml_log = "#{@test_out_dir}/TestResultTrx.xml"
      FileUtils.rm_f(xml_log)

      cli = [
        "dotnet",
        "test",
        @project_file,
        "--logger:trx;LogFileName=TestResultTrx.xml",
        "--results-directory", @test_out_dir,
      ]
    else
      xml_log = "#{@test_out_dir}/TestResult.xml"
      FileUtils.rm_f(xml_log)

      cli = [
        detect_runner,
        "--result=#{xml_log}",
        @out_dll
      ]
      cli.unshift("mono") if @is_mono
    end

    run_and_tee(
      {"CSHARP_TEST_SRC_PATH" => "src"},
      cli,
      "#{@test_out_dir}/test_run.stdout"
    )

    File.exist?(xml_log)
  end

#  private
  def convert_slashes(list)
    sort_list_with_bare(list).map { |f|
      if is_bare?(f)
        # bare file, return as is, no slashes expected anyway
        f
      else
        f.gsub(/\//, '\\')
      end
    }
  end
end

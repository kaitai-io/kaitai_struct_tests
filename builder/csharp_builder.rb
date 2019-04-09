require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class CSharpBuilder < PartialBuilder
  def initialize
    super

    @mode = :make_posix
    @mode = :msbuild_windows if ENV['APPVEYOR']

    @spec_dir = 'spec/csharp/kaitai_struct_csharp_tests'
    @compiled_dir = 'compiled/csharp'
    @project_file = "#{@spec_dir}/kaitai_struct_csharp_tests.csproj"
    @project_template = "#{@spec_dir}/kaitai_struct_csharp_tests.csproj.in"

    @test_out_dir = "#{@config['TEST_OUT_DIR']}/csharp"

    detect_tools
  end

  def detect_tools
    # msbuild
    if system("msbuild /version")
      @msbuild = 'msbuild'
    elsif system("xbuild /version")
      @msbuild = 'xbuild'
    else
      raise 'Unable to find msbuild/xbuild, bailing out'
    end

    # If we're running in AppVeyor, add extra logger args
    @msbuild_args = []
    if ENV['APPVEYOR']
      @msbuild_args << '/logger:C:\Program Files\AppVeyor\BuildAgent\Appveyor.MSBuildLogger.dll'
    end
  end

  def list_mandatory_files
    convert_slashes([
      'Properties/AssemblyInfo.cs',
      'CommonSpec.cs',
    ])
  end

  def list_disposable_files
    orig_pwd = Dir.pwd

    rel_files = Dir.glob("#{@spec_dir}/tests/**/*.cs") + Dir.glob("#{@compiled_dir}/**/*.cs")
    abs_files = rel_files.map { |fn| "#{orig_pwd}/#{fn}" }
    convert_slashes(abs_files)
  end

  def create_project(mand_files, disp_files)
    tmpl = File.read(@project_template)
    files_xml = (mand_files + disp_files).map { |x| "    <Compile Include=\"#{x}\" />" }.join("\n")
    project = tmpl.gsub(/%%%FILES%%%/, files_xml)
    File.write(@project_file, project)
  end

  def build_project(log_file)
    cli = [@msbuild] + @msbuild_args + ["#{@spec_dir}/kaitai_struct_csharp_tests.csproj"]
    puts "run-csharp: running msbuild: #{cli.inspect}"
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
          list << filename
        end
      }
    }

    convert_slashes(list)
  end

  private
  def convert_slashes(list)
    list.sort.map { |f| f.gsub(/\//, '\\') }
  end
end

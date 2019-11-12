require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class CSharpBuilder < PartialBuilder
  def initialize
    super

    @spec_dir = 'spec/csharp/kaitai_struct_csharp_tests'
    @packages_dir = 'spec/csharp/packages'
    @compiled_dir = 'compiled/csharp'
    @project_file = "#{@spec_dir}/kaitai_struct_csharp_tests.csproj"

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

    # New csproj format requires NuGet restore before build
    @msbuild_args << '/restore'

    # If mono is available, use it
    @is_mono = system("mono --version") ? true : false
  end

  def list_mandatory_files
    convert_slashes([
      'CommonSpec.cs',
    ])
  end

  def list_disposable_files
    rel_files = Dir.glob("#{@spec_dir}/tests/**/*.cs") + Dir.glob("#{@compiled_dir}/**/*.cs")
    abs_files = rel_files.map { |fn| File.absolute_path(fn) }
    convert_slashes(abs_files)
  end

  def create_project(mand_files, disp_files)
    Dir[@compiled_dir + "/**/*.{cs}"].each do |original_parser_filepath| 
      new_parser_filepath = original_parser_filepath.gsub(@compiled_dir, @spec_dir + "/parsers/")
      copy_with_path(original_parser_filepath, new_parser_filepath);
    end
    @project_file
  end

  def copy_with_path(src, dst)
    FileUtils.mkdir_p(File.dirname(dst))
    FileUtils.cp(src, dst)
  end

  def build_project(log_file)
    cli = [@msbuild] + @msbuild_args + ["#{@spec_dir}/kaitai_struct_csharp_tests.csproj"]
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

          if filename =~ /^tests\/([^\/].*)$/
            # No absolute path, this happens for some files in xbuild
            # mono builds - treat that as bare files
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

  def run_tests
    xml_log = "#{@test_out_dir}/TestResult.xml"
    FileUtils.mkdir_p(@test_out_dir)
    FileUtils.rm_f(xml_log)

    cli = [
      "#{@packages_dir}/NUnit.ConsoleRunner.3.4.1/tools/nunit3-console.exe",
      "--result=#{xml_log}",
      "#{@spec_dir}/bin/Debug/kaitai_struct_csharp_tests.dll",
    ]

    cli.unshift("mono") if @is_mono

    run_and_tee(
      {"CSHARP_TEST_SRC_PATH" => "src"},
      cli,
      "#{@test_out_dir}/test_run.stdout"
    )

    File.exists?(xml_log)
  end

#  private
  def convert_slashes(list)
    list.sort { |a, b|
      aa = is_bare?(a) ? a[1] : a
      bb = is_bare?(b) ? b[1] : b
      aa <=> bb
    }.map { |f|
      if is_bare?(f)
        # bare file, return as is, no slashes expected anyway
        f
      else
        f.gsub(/\//, '\\')
      end
    }
  end
end

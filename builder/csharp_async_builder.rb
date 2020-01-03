require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class CSharpBuilder < PartialBuilder
  def initialize
    super

    @spec_dir = 'spec/csharp_async/Kaitai.Struct.Tests'
    @compiled_dir = 'compiled/csharpasync'
    @project_file = "#{@spec_dir}/Kaitai.Struct.Tests.csproj"
    @project_template = "#{@spec_dir}/Kaitai.Struct.Tests.csproj.in"

    @test_out_dir = "#{@config['TEST_OUT_DIR']}/csharp_async"

    detect_tools
  end

  def detect_tools   
    if !system("dotnet build /version")
      raise 'Unable to find dotnet cli, bailing out'
    end
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
    tmpl = File.read(@project_template)
    files_xml = (mand_files + disp_files).map { |x| "    <Compile Include=\"#{x}\" />" }.join("\n")
    project = tmpl.gsub(/%%%FILES%%%/, files_xml)
    File.write(@project_file, project)
    @project_file
  end

  def build_project(log_file)
    cli = "dotnet build #{@spec_dir}/Kaitai.Struct.Tests.csproj"
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

  def run_tests
    xml_log = "#{@test_out_dir}/TestResult.xml"
    FileUtils.mkdir_p(@test_out_dir)
    FileUtils.rm_f(xml_log)

    cli = "dotnet test #{@spec_dir}/bin/Debug/Kaitai.Struct.Tests.dll --test-adapter-path:. --logger:\"nunit;LogFilePath=#{xml_log}\""

    run_and_tee(
      {"CSHARP_TEST_SRC_PATH" => "src"},
      cli,
      "#{@test_out_dir}/test_run.stdout"
    )

    File.exists?(xml_log)
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

require 'fileutils'
require 'set'

require_relative 'partial_builder'
require_relative 'shellconfig'

class JavaBuilder < PartialBuilder
  def initialize
    super

    @spec_dir     = File.join('spec', 'java', 'src')
    @compiled_dir = File.join("#{@config['FORMATS_COMPILED_DIR']}", 'java')
    @project_file = File.join("#{@compiled_dir}", 'src_list.txt')

    @java_runtime_dir = @config['JAVA_RUNTIME_DIR'] or raise "JAVA_RUNTIME_DIR is undefined"
    @java_runtime_dir = File.absolute_path(@java_runtime_dir)

    @java_testng_jar = @config['JAVA_TESTNG_JAR'] or raise "JAVA_TESTNG_JAR is undefined"
    @java_testng_jar = @java_testng_jar.split(/(?<=\.jar):/).each { |x| File.absolute_path(x) }
    @java_testng_jar = @java_testng_jar.join(File::PATH_SEPARATOR)

    @java_classes_dir = "#{@compiled_dir}/bin"
    @java_classes_dir = File.absolute_path(@java_classes_dir)
    @java_classpath   = [@java_classes_dir, @java_testng_jar].join(File::PATH_SEPARATOR)

    @test_out_dir = "#{@config['TEST_OUT_DIR']}/java"
    @test_out_dir = File.absolute_path(@test_out_dir)
  end

  def list_mandatory_files
    Dir.glob(File.join("#{@java_runtime_dir}", 'src', 'main', '**/*.java'))
  end

  def list_disposable_files
    Dir.glob(File.join("#{@spec_dir}", '**/*.java')) +
    Dir.glob(File.join("#{@compiled_dir}", 'src', '**/*.java'))
  end

  def create_project(mand_files, disp_files)
    File.open(@project_file, 'w') { |f|
      (mand_files + disp_files).each { |l| f.puts "\"#{l}\"" }
    }
    @project_file
  end

  def build_project(log_file)
    FileUtils.mkdir_p(@java_classes_dir)
    cli = [
      'javac',
      '-encoding', 'UTF-8',
      '-cp',  @java_classpath,
      '-d',   @java_classes_dir,
      "@#{@project_file}",
    ]
    run_and_tee({}, cli, log_file).exitstatus
  end

  def parse_failed_build(log_file, disp_files)
    list = Set.new

    File.open(log_file, 'r') { |f|
      f.each_line { |l|
        l.chomp!
        if l =~ /^*(.+?\.java):(\d+): error: (.*?)$/
          filename = $1.gsub('\\', '/')
          #row = $2
          #msg = $3
          list << filename
        end
      }
    }

    list
  end

  def file_to_test(filename)
    # File.basename only forwards with forward slashes, so we normalize for that first
    fn = File.basename(filename.gsub(/\\/, '/'))
    if fn =~ /^Test(.*)\.java$/
      return [:spec, $1]
    else
      return [:format, fn.gsub(/\.java$/, '')]
    end
  end

  def run_tests
    orig_dir = Dir.pwd
    FileUtils.mkdir_p(@test_out_dir)
    FileUtils.rm_rf(File.join("#{@test_out_dir}", 'junitreports'))

    cli = [
      'java',
      '-cp', @java_classpath,
      'org.testng.TestNG',
      '-d', @test_out_dir, File.absolute_path(File.join('spec', 'java', 'testng.xml'))
    ]
    out_log = File.join(@test_out_dir, 'test_run.stdout')

    # Java uses "../../src" to locate binary input files, so we change
    # working directory prior to running tests to match that
    Dir.chdir(File.join('spec', 'java'))
    run_and_tee({}, cli, out_log)
    Dir.chdir(orig_dir)

    File.exists?(File.join(@test_out_dir, 'junitreports'))
  end
end

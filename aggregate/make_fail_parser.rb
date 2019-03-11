require_relative 'test_parser'

def underscore_to_ucamelcase(s)
  s.split(/_/).map { |x| x.capitalize }.join
end

##
# Parses fails of C++ compilation process as evidenced in `make.stderr`.
class MakeFailParser < TestParser
  def initialize(fn)
    @fn = fn
  end

  def each_test
    buf = []
    begin
      File.open(@fn, 'r') { |f|
        f.each_line { |l|
          l.chomp!
          if l =~ /^make\[\d+\]: \*\*\* \[(.*?)\] Error 1$/
            # might be just obj file or
            # "CMakeFiles/ks_tests.dir/build.make:1181: CMakeFiles/ks_tests.dir/blah/blah/switch_integers2.cpp.o"
            obj_file = $1
            obj_file.gsub!(/^.*: /, '')
            obj_file = File.basename(obj_file).gsub(/\.cpp\.o$/, '')
            yield get_test_result(obj_file, buf)
            buf = []
          else
            buf << l
          end
        }
      }
    rescue Errno::ENOENT => e
      $stderr.puts e.inspect
    end
  end

  def get_test_result(obj_file, buf)
    if obj_file.start_with?('test_')
      name = obj_file[5..-1]
      what_failed = :spec_build_failed
    else
      name = obj_file
      what_failed = :format_build_failed
    end
    name = underscore_to_ucamelcase(name)
    failure = TestResult::Failure.new(nil, nil, buf.join("\n"), nil)
    TestResult.new(name, what_failed, 0, failure)
  end
end

#MakeFailParser.new('../test_out/cpp_stl_11/make.stderr').each_test { |t| p t }
#MakeFailParser.new('../../../ci_artifacts/test_out/cpp_stl_11/make.stderr').each_test { |t| p t }

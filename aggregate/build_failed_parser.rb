require_relative 'test_parser'

# Parses fails of unified *Builder compilation process as evidenced by `build_failed_tests.txt`.
class BuildFailedParser < TestParser
  def initialize(fn, is_luc = false)
    @fn = fn
    @is_luc = is_luc
  end

  def each_test
    buf = []
    begin
      File.open(@fn, 'r') { |f|
        f.each_line { |l|
          l.chomp!
          typ, name = l.split(/\t/)

          what_failed = case typ
          when 'format' then :format_build_failed
          when 'spec' then :spec_build_failed
          end

          name = underscore_to_ucamelcase(name) if @is_luc

          yield TestResult.new(name, what_failed, nil)
        }
      }
    rescue Errno::ENOENT => e
      $stderr.puts e.inspect
    end
  end
end

#BuildFailedParser.new('../../../ci_artifacts/test_out/cpp_stl_11/build_failed_tests.txt').each_test { |t| p t }

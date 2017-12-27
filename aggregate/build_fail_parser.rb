require 'set'

class BuildFailParser
  def initialize(dir)
    @dir = dir
  end

  # Special hack: ignore some imported files which are definitely
  # *not* distinct tests per se, but might fail compilation while
  # being imported from main test files.
  BLACKLIST_TESTS = Set.new([
    'Imported1',
    'ImportedAndAbs',
    'ImportedAndRel',
  ])

  def each_test
    fn = "#{@dir}/build.fails"
    File.open(fn, 'r') { |f|
      f.each_line { |l|
        case l.chomp
        when /^Test(.*?)\.java$/
          name = $1
          yield TestResult.new(name, :spec_build_failed, 0, nil)
        when /^(.*?)\.java$/
          name = $1
          yield TestResult.new(name, :format_build_failed, 0, nil)
        when /^(.*?)\.go:(\d+): (.*?)$/
          fn = $1
          name = underscore_to_ucamelcase(File.basename(fn))

          # Ignore files that are definitely not tests
          next if BLACKLIST_TESTS.include?(name)

          fail = TestResult::Failure.new(
            fn,
            $2,
            $3,
            nil
          )
          yield TestResult.new(name, :format_build_failed, 0, fail)
        end
      }
    } if FileTest::exists?(fn)
  end
end

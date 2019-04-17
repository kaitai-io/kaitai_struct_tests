require_relative 'test_parser'

# Simplest test parser one can imagine: just a plain text file with a
# list of tests that passed, one per line.
class TextListParser < TestParser
  def initialize(fn)
    @doc = File.readlines(fn)
  end

  def each_test
    @doc.each { |name|
      name = underscore_to_ucamelcase(name.chomp)
      yield TestResult.new(name, :passed, 0, nil)
    }
  end
end

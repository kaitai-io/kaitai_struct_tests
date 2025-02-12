require_relative 'test_parser'

require 'set'

# Simplest test parser one can imagine: just a plain text file with a
# list of tests that passed, one per line.
class TextListParser < TestParser
  def initialize(fn)
    @doc = File.readlines(fn)
  end

  # We don't have a list of *specs*, we just grab all the files we've
  # ended up with. Some of them are parts of tests, and not the
  # individual tests per se.
  BLACK_LIST = Set.new([
    'Imported1',
    'Imported2',
    'ImportedAndAbs',
    'ImportedAndRel',
    'ImportedRoot',
    'ImportsCircularB',
    'OpaqueExternalType02Child',
    'ParamsPassStruct',
    'VlqBase128Le',
  ])

  def each_test
    @doc.each { |name|
      name = underscore_to_ucamelcase(name.chomp)
      next if BLACK_LIST.include?(name)
      yield TestResult.new(name, :passed, nil)
    }
  end
end

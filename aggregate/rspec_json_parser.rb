require 'json'

require_relative 'test_parser'

class RSpecJSONParser < TestParser
  def initialize(fn)
    @doc = JSON.parse(File.read(fn))
  end

  def each_test
    @doc['examples'].each { |t|
      raise "Unable to parse full_description: \"#{t['full_description']}\"" unless t['full_description'] =~ /^(\S+) parses test properly$/
      class_name = $1

      raise "Unable to parse file_path: \"#{t['file_path']}\"" unless File.basename(t['file_path']) =~ /^(.*)_spec.rb$/
      top_name = $1

      # TODO: check that top_name matches class_name

      failure = nil
      if t['status'] != 'passed'
        failure = TestResult::Failure.new(
          t['file_path'],
          t['line_number'],
          t['exception']['message'],
          t['exception']['backtrace'].join("\n")
        )
      end

      tr = TestResult.new(class_name, t['status'].to_sym, t['run_time'], failure)
      yield tr
    }
  end
end

require 'rexml/document'

require_relative 'test_parser'

class NUnitXMLParser < TestParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.elements.each('/test-run/test-suite/test-suite/test-suite/test-case') { |tc|
      name = tc.attribute('name').value

      raise "Unable to parse name: \"#{name}\"" unless name =~ /^Test(.*?)$/
      name = $1

      result = tc.attribute('result').value
      failure = nil
      status = case result
               when 'Passed'
                 :passed
               when 'Failed'
                 classname = tc.attribute('classname')
                 classname = classname.value if classname

                 methodname = tc.attribute('methodname')
                 methodname = methodname.value if methodname

                 filename = "#{classname || '?'}:#{methodname || '?'}"

                 failure_xml = tc.elements['failure']
                 failure_msg = failure_xml.elements['message'].text
                 failure_trace = failure_xml.elements['stack-trace'].text

                 failure = TestResult::Failure.new(
                   filename,
                   nil,
                   failure_msg,
                   failure_trace
                 )
                 :failed
               else
                 raise "Unable to parse result: #{result.inspect}"
               end

      tr = TestResult.new(name, status, tc.attribute('duration').value.to_f, failure)
      yield tr
    }
  end
end

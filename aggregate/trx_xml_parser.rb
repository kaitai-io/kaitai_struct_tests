require 'rexml/document'

require_relative 'test_parser'

class TRXXMLParser < TestParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.elements.each('/TestRun/Results/UnitTestResult') { |tc|
      name = tc.attribute('testName').value

      raise "Unable to parse name: \"#{name}\"" unless name =~ /^Test(.*?)$/
      name = $1

      result = tc.attribute('outcome').value
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

                 failure_xml = tc.elements['Output'].elements['ErrorInfo']
                 failure_msg = failure_xml.elements['Message'].text
                 failure_trace = failure_xml.elements['StackTrace'].text

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

      duration = parse_duration(tc.attribute('duration').value)

      tr = TestResult.new(name, status, duration, failure)
      yield tr
    }
  end

  def parse_duration(dur_str)
    # "00:00:00.0004430"
    raise "Unable to parse duration: #{dur_str.inspect}" unless dur_str =~ /^(\d\d):(\d\d):(\d+\.\d+)$/
    h = $1.to_i
    m = $2.to_i
    s = $3.to_f

    return h * 60 * 60 + m * 60 + s
  end
end

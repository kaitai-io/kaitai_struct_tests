require 'rexml/document'

class NUnitXMLParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.elements.each('/test-run/test-suite/test-suite/test-suite/test-case') { |tc|
      name = tc.attribute('name').value

      raise "Unable to parse name: \"#{name}\"" unless name =~ /^Test(.*?)$/

      result = tc.attribute('result').value
      status = case result
               when 'Passed'
                 :passed
               else
                 raise "Unable to parse result: #{result.inspect}"
               end

      failure = nil
#        failure_xml = tc.elements['failure'] || tc.elements['error']
#
#        if failure_xml.nil?
#          status = :passed
#          failure = nil
#        else
#          status = :failed
#          failure_msg = failure_xml.attribute('message')
#          failure_msg = failure_msg.value if failure_msg
#          failure = TestResult::Failure.new(
#            nil,
#            nil,
#            failure_msg,
#            failure_xml.children
#          )
#        end

      tr = TestResult.new(name, status, tc.attribute('duration').value.to_f, failure)
      yield tr
    }
  end
end

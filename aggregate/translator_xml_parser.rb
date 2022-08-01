require 'rexml/document'

class TranslatorXMLParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.root.elements.each('testcase') { |tc|
      name = tc.attribute('name').value

      raise "Unable to parse name: \"#{name}\"" unless name =~ /^(.*?):(.*)$/
      lang = $1
      expr = $2

      failure_xml = tc.elements['failure']

      if failure_xml.nil?
        status = :passed
        failure = nil
      else
        status = :failed
        failure_msg = failure_xml.attribute('message')
        failure_msg = failure_msg.value if failure_msg
        failure_trace = (failure_xml.texts.map {|t| t.value }).join('').strip

        status = :no_spec if failure_msg == 'no expected result'
        failure = TestResult::Failure.new(
          nil,
          nil,
          failure_msg,
          failure_trace
        )
      end

      tr = TestResult.new(expr, status, tc.attribute('time').value.to_f, failure)
      yield lang, tr
    }
  end
end

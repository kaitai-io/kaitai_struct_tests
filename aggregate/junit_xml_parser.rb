require 'rexml/document'

def camelcase_to_underscore(s)
  r = ''

  s.each_char { |c|
    if ('A'..'Z').include?(c)
      r << '_'
      r << c.downcase
    else
      r << c
    end
  }

  r.gsub(/^_/, '')
end

class JUnitXMLParser
  def initialize(fn)
    @docs = if File.directory?(fn)
      Dir.glob("#{fn}/*.xml").map { |x|
        REXML::Document.new(File.read(x))
      }
    else
      [REXML::Document.new(File.read(fn))]
    end
  end

  def each_test
    @docs.each { |doc|
      doc.root.elements.each('testcase') { |tc|
        name = tc.attribute('name').value

        raise "Unable to parse name: \"#{name}\"" unless name =~ /^test(.*?)$/
        name = $1
        uname = camelcase_to_underscore(name)

        failure_xml = tc.elements['failure']
        if failure_xml.nil?
          status = :passed
          failure = nil
        else
          status = :failed
          failure = TestResult::Failure.new(
            nil,
            nil,
            failure_xml.attribute('message').value,
            failure_xml.children
          )
        end

        tr = TestResult.new(uname, status, tc.attribute('time').value.to_f, failure)
        yield tr
      }
    }
  end
end

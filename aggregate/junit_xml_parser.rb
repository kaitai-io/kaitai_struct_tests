require 'rexml/document'

def underscore_to_ucamelcase(s)
  s.split(/_/).map { |x| x.capitalize }.join
end

class JUnitXMLParser
  def initialize(fn)
    @docs = if not File.exists?(fn)
      []
    elsif File.directory?(fn)
      Dir.glob("#{fn}/*.xml").map { |x|
        REXML::Document.new(File.read(x))
      }
    else
      [REXML::Document.new(File.read(fn))]
    end
  end

  def each_test
    @docs.each { |doc|
      next unless doc.root
      doc.elements.each('//testcase') { |tc|
        name = tc.attribute('name').value

        if tc.attribute('classname') and tc.attribute('classname').value =~ /^spec_perl_Test(.*?)_t$/
          # Perl output, extract test from classname
          name = $1
        elsif name == 'parses test properly'
          # Mocha output, use classname
          name = tc.attribute('classname').value
        elsif name =~ /^Test.*\.test_/
          # Lua output, use classname
          name = tc.attribute('classname').value.gsub(/^Test/, '')
        else
          raise "Unable to parse name: \"#{name}\"" unless name =~ /^[Tt]est(.*?)$/
          if $1[0] == '_'
            name = underscore_to_ucamelcase($1)
          else
            name = $1
          end
        end

        failure_xml = tc.elements['failure'] || tc.elements['error']

        if failure_xml.nil?
          status = :passed
          failure = nil
        else
          status = :failed
          failure_msg = failure_xml.attribute('message')
          failure_msg = failure_msg.value if failure_msg
          failure = TestResult::Failure.new(
            nil,
            nil,
            failure_msg,
            failure_xml.children
          )
        end

        tr = TestResult.new(name, status, tc.attribute('time').value.to_f, failure)
        yield tr
      }

      # Pick up PHP empty testsuites = skipped tests
      doc.elements.each('//testsuite') { |ts|
        if ts.children.size == 0
          name = ts.attribute('name').value.gsub(/^.*\\/, '').gsub(/Test$/, '')

          tr = TestResult.new(name, :skipped, 0, nil)
          yield tr
        end
      }
    }
  end
end

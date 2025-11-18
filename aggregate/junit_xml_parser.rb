require_relative 'test_parser'

require 'rexml/document'

class JUnitXMLParser < TestParser
  def initialize(fn, include_classname = false)
    @include_classname = include_classname
    @docs = if not File.exist?(fn)
      []
    elsif File.directory?(fn)
      Dir.glob("#{fn}/*.xml").map { |x|
        REXML::Document.new(File.read(x).encode('UTF-8', :invalid=>:replace, :replace=>"?"))
      }
    else
      [REXML::Document.new(File.read(fn).encode('UTF-8', :invalid=>:replace, :replace=>"?"))]
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
        elsif name =~ /^t(?!est)(.*?)$/
          # Nim output
          name = underscore_to_ucamelcase($1)
        elsif name =~ /^test_.*::test_(.*?)$/
          # Rust output
          name = underscore_to_ucamelcase($1)
        elsif tc.attribute('classname') and tc.attribute('classname').value =~ /^\/(.*?) test$/
          # Julia output
          name = $1
        else
          raise "Unable to parse name: \"#{name}\"" unless name =~ /^[Tt]est(.*?)$/
          name = $1
          if name[0] == '_'
            name = underscore_to_ucamelcase(name[1..-1])
          end

          if @include_classname
            classname = tc.attribute('classname').value
            if classname =~ /\.Test([^.]*)$/
              classname = $1
              name = "#{classname}.#{name}"
            else
              warn "Unable to parse classname #{classname.inspect} at #{tc.xpath}"
            end
          end
        end

        failure_xml = tc.elements['failure'] || tc.elements['error']

        if failure_xml.nil?
          skipped_xml = tc.elements['skipped']

          if skipped_xml.nil?
            status = :passed
            failure = nil
          else
            status = :skipped
            failure_msg = skipped_xml.attribute('message')
            failure = TestResult::Failure.new(
              nil,
              nil,
              failure_msg,
              nil
            )
          end
        else
          # Until TestNG 7.4.0, throwing a SkipException is reported as an error
          # (not as a skip, as it should be) in the JUnit XML report by TestNG -
          # see <https://github.com/testng-team/testng/issues/1632>. Since we
          # are using an older version of TestNG for compatibility with Java 8,
          # we have to fix the reported status ourselves.
          if failure_xml.attribute('type') && failure_xml.attribute('type').value == 'org.testng.SkipException'
            status = :skipped
          else
            status = :failed
          end
          failure_msg = failure_xml.attribute('message') || failure_xml.attribute('type')
          failure_msg = failure_msg.value if failure_msg
          failure_trace = (failure_xml.texts.map {|t| t.value }).join('').strip
          failure = TestResult::Failure.new(
            nil,
            nil,
            failure_msg,
            failure_trace
          )
        end

        tr = TestResult.new(name, status, tc.attribute('time').value.to_f, failure)
        yield tr
      }

      # Pick up PHP empty testsuites = skipped tests
      doc.elements.each('//testsuite') { |ts|
        if ts.children.size == 0
          name = ts.attribute('name').value.gsub(/^.*\\/, '').gsub(/Test$/, '')

          tr = TestResult.new(name, :skipped, nil)
          yield tr
        elsif ts.attribute('errors') && ts.attribute('errors').value.to_f != 0 && ts.attribute('name').value =~ /^\/(.*?) test$/
          # Pick up Julia errored tests
          name = $1
          error_element = ts.elements['error']
          error_message = error_element.attribute('message').value if error_element
          error_trace = error_element.text.strip if error_element

          tr = TestResult.new(name, :error, 0, TestResult::Failure.new(nil, nil, error_message, error_trace))
          yield tr
        end
      }
    }
  end
end

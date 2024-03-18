require 'rexml/document'

require_relative 'test_parser'

class BoostTestParser < TestParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    @doc.root.elements.each('TestSuite') { |ts|
      ts.elements.each('TestCase') { |tc|
        name = tc.attribute('name').value

        raise "Unable to parse name: \"#{name}\"" unless name =~ /^test_(.*?)$/
        name = underscore_to_ucamelcase($1)

        failures = []
        tc.elements.each('Error') { |err|
          failures << TestResult::Failure.new(
            err.attribute('file'),
            err.attribute('line'),
            err.text,
            nil
          )
        }
        tc.elements.each('Exception') { |err|
          failures << TestResult::Failure.new(
            err.attribute('file'),
            err.attribute('line'),
            err.text,
            nil
          )
        }

        if failures.empty?
          status = :passed
          failure = nil
        else
          status = :failed
          failure = failures[0]
        end

        tr = TestResult.new(name, status, tc.elements['TestingTime'].text.to_f, failure)
        yield tr
      }
    }
  end

  def aborted_tests
    tests = []
    # An "aborted" test means that "a critical error is detected", which means
    # that "the test execution is aborted and all remaining tests are discarded"
    # (see https://www.boost.org/doc/libs/1_84_0/libs/test/doc/html/boost/unit_test/test_observer.html#idm46380-bb).
    # So in practice there will probably never be more than 1 aborted test (per
    # log file), but we don't rely on that here and return an array anyway.
    each_aborted_test { |t| tests << t }
    tests
  end

  def each_aborted_test
    r = @doc.root
    r.elements.each('TestSuite/TestCase') { |tc|
      next if tc.elements['Exception'].nil?

      aborted = false

      tc.elements.each('Message') { |msg|
        next unless msg.text == 'Test is aborted'

        file_attr = msg.attribute('file')
        next if file_attr.nil?

        file = file_attr.value
        next unless file.end_with?('/boost/test/impl/unit_test_log.ipp')

        aborted = true
        break
      }

      next unless aborted

      name = tc.attribute('name').value
      yield name
    }
  end
end

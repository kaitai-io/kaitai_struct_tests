require 'rexml/document'

require_relative 'test_parser'

ONE_MICROSECOND_IN_SECONDS = 1e-6

class BoostTestParser < TestParser
  def initialize(fn, glob = false)
    files = glob ? Dir.glob(fn) : [fn]
    @docs = files.map { |f| REXML::Document.new(File.read(f)) }
  end

  def each_test
    @docs.each do |doc|
      r = doc.root
      next unless r

      r.elements.each('TestSuite') { |ts|
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

          # `<TestCase>` elements of tests that are disabled (using the
          # `--run_test=!<test_name>` command-line option) because they aborted
          # (typically due to a memory access violation, which terminates the
          # entire test suite) in a previous run attempt look like this in the
          # XML log:
          #
          # ```xml
          #     <TestCase name="test_debug_array_user" skipped="yes" reason="disabled"/>
          # ```
          #
          # As you can see, they don't contain any `<TestingTime>` tag, so we
          # need to make sure we handle this case gracefully.
          testing_time_node = tc.elements['TestingTime']
          testing_time =
            if testing_time_node.nil?
              nil
            else
              # `<TestingTime>` contains the elapsed time in microseconds, see:
              #
              # 1. https://github.com/boostorg/test/blob/d2895ebfdfdf16074c58c9801d53e190c4654fcb/include/boost/test/impl/xml_log_formatter.ipp#L100
              # 2. https://github.com/boostorg/test/blob/d2895ebfdfdf16074c58c9801d53e190c4654fcb/include/boost/test/unit_test_log_formatter.hpp#L160
              testing_time_node.text.to_f * ONE_MICROSECOND_IN_SECONDS
            end

          tr = TestResult.new(name, status, testing_time, failure)
          yield tr
        }
      }
    end
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
    @docs.each do |doc|
      r = doc.root
      next unless r

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
end

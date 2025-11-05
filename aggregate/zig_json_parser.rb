# frozen_string_literal: true

require_relative 'test_parser'

class ZigJSONParser < TestParser
  def initialize(report_json_fn, stderr_log_fn)
    report = JSON.parse(File.read(report_json_fn, encoding: Encoding::UTF_8))
    report_tests = report['tests']
    test_name_message_pairs = ZigJSONParser.parse_messages_from_stderr(stderr_log_fn)

    num_tests_in_report = report_tests.length
    num_tests_on_stderr = test_name_message_pairs.length
    if num_tests_in_report != num_tests_on_stderr
      raise "Found #{num_tests_in_report} tests in #{report_json_fn}, " \
            "but #{num_tests_on_stderr} tests in #{stderr_log_fn}"
    end

    test_result_message_pairs = report_tests.zip(test_name_message_pairs)
    test_result_message_pairs.each_with_index do |(test_result, (test_name, message)), i|
      unless test_result['name'] == test_name
        raise "Test names at index #{i} do not match " \
              "(#{test_result['name'].inspect} != #{test_name.inspect})"
      end
      test_result['message'] = message

      test_result['message'].chomp!
      test_result['error']['trace'].chomp! unless test_result['error'].nil?
    end

    @tests = report_tests
  end

  def each_test
    @tests.each do |test|
      test_name = test['name']
      raise "Unable to parse test name #{test_name.inspect}" unless test_name =~ /\.([^.]*?)$/
      name = $1

      if test['error'].nil?
        status = :passed
        failure = nil
      else
        err = test['error']
        status =
          if err['type'] == 'SkipZigTest'
            :skipped
          else
            :failed
          end
        message = err['type']
        message << ": #{test['message']}" unless test['message'].empty?
        failure = TestResult::Failure.new(nil, nil, message, err['trace'])
      end

      tr = TestResult.new(name, status, nil, failure)
      yield tr
    end
  end

  # @param stderr_log_fn [String]
  def self.parse_messages_from_stderr(stderr_log_fn)
    test_message_pairs = []
    File.open(stderr_log_fn, 'r', encoding: Encoding::UTF_8) do |f|
      message = +''
      test_name = nil
      f.each_line { |l|
        # Since the `std.testing.expectEqualStrings` function prints the actual
        # and expected strings verbatim to stderr, and "strings" in Zig are just
        # byte arrays with no guarantee of valid UTF-8 content, the string `l`
        # may contain invalid UTF-8 byte sequences. In this case, we would get
        # an error `in 'Regexp#===': invalid byte sequence in UTF-8
        # (ArgumentError)`, which would prevent the ci.json file from being
        # generated.
        #
        # To prevent this, we use the `String#scrub!` method to replace invalid
        # UTF-8 byte sequences with the replacement character U+FFFD.
        l.scrub!
        case l
        when /^=@= ZIG_TEST_START (.*) ~#~\n$/
          if test_name.nil?
            warn "[ZigJSONParser] warning: found #{message.inspect} on stderr before the first test start marker" unless message.empty?
          else
            test_message_pairs << [test_name, message]
            message = +''
          end
          test_name = $1
        else
          message << l
        end
      }
      if test_name.nil?
        warn "[ZigJSONParser] warning: found #{message.inspect} on stderr and no test start marker" unless message.empty?
      else
        test_message_pairs << [test_name, message]
      end
    end
    test_message_pairs
  end
end

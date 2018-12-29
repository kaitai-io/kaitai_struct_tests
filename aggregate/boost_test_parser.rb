require 'rexml/document'

require_relative 'test_parser'

def underscore_to_ucamelcase(s)
  s.split(/_/).map { |x| x.capitalize }.join
end

class BoostTestParser < TestParser
  def initialize(fn)
    @doc = REXML::Document.new(File.read(fn))
  end

  def each_test
    # Boost actually uses two slightly different formats: old and
    # new. Instead of trying to detect which one we have, we actually
    # just try both.
    each_from_old_boost_format { |tr| yield tr }
    each_from_new_boost_format { |tr| yield tr }
  end

  def each_from_old_boost_format
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

  def each_from_new_boost_format
    @doc.root.elements.each('testcase') { |tc|
      name = tc.attribute('name').value

      raise "Unable to parse name: \"#{name}\"" unless name =~ /^test_(.*?)$/
      name = underscore_to_ucamelcase($1)

      failures = []
      tc.elements.each('failure') { |err|
        failures << TestResult::Failure.new(
          nil, # filename
          nil, # line
          err.text,
          nil
        )
      }

      tc.elements.each('error') { |err|
        failures << TestResult::Failure.new(
          nil, # filename
          nil, # line
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

      tr = TestResult.new(name, status, tc.attribute('time').value.to_f, failure)
      yield tr
    }
  end
end

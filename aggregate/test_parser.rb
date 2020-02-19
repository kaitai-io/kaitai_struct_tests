require_relative 'test_result'

# Common helper method: converts `lower_under_score` to `UpperCamelCase`
def underscore_to_ucamelcase(s)
  s.split(/_/).map { |x| x.capitalize }.join
end

class TestParser
  def each_test
    raise 'Abstract method'
  end

  def to_h
    r = {}
    each_test { |t|
      if (not r.key?(t.name)) || r[t.name]['status'] == 'passed'
        r[t.name] = t.to_h
      end
    }
    r
  end

  def to_json
    JSON.pretty_generate(to_h)
  end
end

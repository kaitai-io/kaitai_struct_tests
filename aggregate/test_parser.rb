require_relative 'test_result'

class TestParser
  def each_test
    raise 'Abstract method'
  end

  def to_h
    r = {}
    each_test { |t|
      r[t.name] = t.to_h
    }
    r
  end

  def to_json
    JSON.pretty_generate(to_h)
  end
end

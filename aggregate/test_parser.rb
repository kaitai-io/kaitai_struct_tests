require_relative 'test_result'

# Common helper method: converts `lower_under_score` to `UpperCamelCase`
def underscore_to_ucamelcase(s)
  s.split(/_/).map { |x| x.capitalize }.join
end

class TestParser
  def each_test
    raise 'Abstract method'
  end

  def to_json
    JSON.pretty_generate(to_enum(:each_test).map { |t|
      h = {'name' => t.name}
      h.merge!(t.to_h)
      h
    })
  end
end

require 'floating_points'

RSpec.describe FloatingPoints do
  it 'parses test properly' do
    r = FloatingPoints.from_file('src/floating_points.bin')
    
    delta = 1e-6

    expect(r.single_value).to be_within(delta).of 1.2345
    expect(r.single_value_be).to be_within(delta).of 1.2345
    expect(r.double_value).to be_within(delta).of 123.456
    expect(r.double_value_be).to be_within(delta).of 123.456
    
    expect(r.single_value_plus_int).to be_within(delta).of 2.2345
    expect(r.single_value_plus_float).to be_within(delta).of 1.7345
    expect(r.double_value_plus_float).to be_within(delta).of 123.506
  end
end

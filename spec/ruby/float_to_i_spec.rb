require 'float_to_i'

RSpec.describe FloatToI do
  it 'parses test properly' do
    r = FloatToI.from_file('src/floating_points.bin')
    
    expect(r.single_value).to eq 0.5
    expect(r.double_value).to eq 0.25
    
    expect(r.single_i).to eq 0
    expect(r.double_i).to eq 0
    expect(r.float1_i).to eq 1
    expect(r.float2_i).to eq 1
    expect(r.float3_i).to eq 1
    expect(r.float4_i).to eq -2
  end
end

RSpec.describe 'ParamsDef' do
  it 'parses test properly' do
    require 'params_def'
    io = Kaitai::Struct::Stream.open('src/term_strz.bin')
    r = ParamsDef.new(io, nil, nil, 5, true)

    expect(r.buf).to eq('foo|b')
    expect(r.trailer).to eq(0x61)
  end
end

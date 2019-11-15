require 'opaque_with_param'

RSpec.describe OpaqueWithParam do
  it 'parses test properly' do
    r = OpaqueWithParam.from_file('src/term_strz.bin')

    expect(r.one.buf).to eq('foo|b')
    expect(r.one.trailer).to eq(0x61)
  end
end

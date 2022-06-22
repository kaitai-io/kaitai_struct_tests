RSpec.describe 'OpaqueWithParam' do
  it 'parses test properly' do
    require 'opaque_with_param'
    require 'params_def' # the opaque type
    r = OpaqueWithParam.from_file('src/term_strz.bin')

    expect(r.one.buf).to eq('foo|b')
    expect(r.one.trailer).to eq(0x61)
  end
end

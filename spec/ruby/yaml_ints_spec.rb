require 'yaml_ints'

RSpec.describe YamlInts do
  it 'parses test properly' do
    r = YamlInts.from_file('src/fixed_struct.bin')

    expect(r.test_u4_dec).to eq 0xffffffff
    expect(r.test_u4_hex).to eq 0xffffffff
    expect(r.test_u8_dec).to eq 0xffffffffffffffff
    expect(r.test_u8_hex).to eq 0xffffffffffffffff
  end
end

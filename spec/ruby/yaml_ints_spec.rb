# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'YamlInts' do
  it 'parses test properly' do
    require 'yaml_ints'
    r = YamlInts.from_file('src/fixed_struct.bin')

    expect(r.test_u4_dec).to eq 4294967295
    expect(r.test_u4_hex).to eq 4294967295
    expect(r.test_u8_dec).to eq 18446744073709551615
    expect(r.test_u8_hex).to eq 18446744073709551615
  end
end

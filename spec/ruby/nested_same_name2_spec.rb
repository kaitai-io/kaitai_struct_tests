require 'nested_same_name2'

RSpec.describe NestedSameName2 do
  it 'parses test properly' do
    r = NestedSameName2.from_file('src/nested_same_name2.bin')

    expect(r.version).to eq 0x42
    expect(r.main_data.main_size).to eq 2
    expect(r.main_data.foo.data1).to eq [0x11, 0x11, 0x11, 0x11].pack('C*')
    expect(r.dummy.dummy_size).to eq 3
    expect(r.dummy.foo.data2).to eq [0x22, 0x22, 0x22, 0x22, 0x22, 0x22].pack('C*')
  end
end

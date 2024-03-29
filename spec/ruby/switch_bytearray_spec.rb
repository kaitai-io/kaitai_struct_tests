# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'SwitchBytearray' do
  it 'parses test properly' do
    require 'switch_bytearray'
    r = SwitchBytearray.from_file('src/switch_opcodes.bin')

    expect(r.opcodes.length).to eq 4
    expect(r.opcodes[0].code).to eq [83].pack('C*')
    expect(r.opcodes[0].body.value).to eq "foobar"
    expect(r.opcodes[1].code).to eq [73].pack('C*')
    expect(r.opcodes[1].body.value).to eq 66
    expect(r.opcodes[2].code).to eq [73].pack('C*')
    expect(r.opcodes[2].body.value).to eq 55
    expect(r.opcodes[3].code).to eq [83].pack('C*')
    expect(r.opcodes[3].body.value).to eq ""
  end
end

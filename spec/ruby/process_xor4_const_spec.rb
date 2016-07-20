require 'process_xor4_const'

RSpec.describe ProcessXor4Const do
  it 'parses test properly' do
    r = ProcessXor4Const.from_file('src/process_xor_4.bin')

    expect(r.key).to eq [0xec, 0xbb, 0xa3, 0x14].pack('C*')
    expect(r.buf).to eq "foo bar"
  end
end

# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ProcessXor4Value' do
  it 'parses test properly' do
    require 'process_xor4_value'
    r = ProcessXor4Value.from_file('src/process_xor_4.bin')

    expect(r.key).to eq [236, 187, 163, 20].pack('C*')
    expect(r.buf).to eq [102, 111, 111, 32, 98, 97, 114].pack('C*')
  end
end

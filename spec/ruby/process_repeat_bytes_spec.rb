# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ProcessRepeatBytes' do
  it 'parses test properly' do
    require 'process_repeat_bytes'
    r = ProcessRepeatBytes.from_file('src/process_xor_4.bin')

    expect(r.bufs[0]).to eq [114, 37, 61, 138, 20].pack('C*')
    expect(r.bufs[1]).to eq [74, 82, 170, 16, 68].pack('C*')
  end
end

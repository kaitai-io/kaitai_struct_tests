require 'process_xor_const'

RSpec.describe ProcessXorConst do
  it 'parses test properly' do
    r = ProcessXorConst.from_file('src/process_xor_1.bin')

    expect(r.key).to eq 0xff
    expect(r.buf).to eq "foo bar"
  end
end

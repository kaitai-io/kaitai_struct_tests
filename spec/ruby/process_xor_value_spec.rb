require 'process_xor_value'

RSpec.describe ProcessXorValue do
  it 'parses test properly' do
    r = ProcessXorValue.from_file('src/process_xor_1.bin')

    expect(r.key).to eq 0xff
    expect(r.buf).to eq "foo bar"
  end
end

require 'debug_0'

RSpec.describe Debug0 do
  it 'parses test properly' do
    r = Debug0.from_file('src/fixed_struct.bin')
    r._read

    expect(r.one).to eq 0x50
    expect(r.array_of_ints).to eq [0x41, 0x43, 0x4b]
  end
end

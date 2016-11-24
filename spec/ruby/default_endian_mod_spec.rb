require 'default_endian_mod'

RSpec.describe DefaultEndianMod do
  it 'parses test properly' do
    r = DefaultEndianMod.from_file('src/fixed_struct.bin')

    expect(r.main.one).to eq 0x4b434150
    expect(r.main.nest.two).to eq -52947
    expect(r.main.nest_be.two).to eq 0x5041434b
  end
end

# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'DefaultEndianMod' do
  it 'parses test properly' do
    require 'default_endian_mod'
    r = DefaultEndianMod.from_file('src/fixed_struct.bin')

    expect(r.main.one).to eq 1262698832
    expect(r.main.nest.two).to eq -52947
    expect(r.main.nest_be.two).to eq 1346454347
  end
end

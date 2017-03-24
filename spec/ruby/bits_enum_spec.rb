require 'bits_enum'

RSpec.describe BitsEnum do
  it 'parses test properly' do
    r = BitsEnum.from_file('src/fixed_struct.bin')

    # 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
    expect(r.one).to eq :animal_platypus
    expect(r.two).to eq :animal_horse
    expect(r.three).to eq :animal_cat
  end
end

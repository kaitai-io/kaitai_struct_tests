require 'enum_invalid'

RSpec.describe EnumInvalid do
  it 'parses test properly' do
    r = EnumInvalid.from_file('src/term_strz.bin')

    expect(r.pet_1).to eq :animal_dog
    expect(r.pet_2).to eq 0x6f
  end
end

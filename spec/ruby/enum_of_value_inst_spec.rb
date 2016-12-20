# coding: utf-8
require 'enum_of_value_inst'

RSpec.describe EnumOfValueInst do
  it 'parses test properly' do
    r = EnumOfValueInst.from_file('src/enum_0.bin')

    expect(r.pet_1).to eq :animal_cat
    expect(r.pet_2).to eq :animal_chicken
    expect(r.pet_3).to eq :animal_dog
    expect(r.pet_4).to eq :animal_dog
  end
end

# coding: utf-8
require 'enum_1'

RSpec.describe Enum1 do
  it 'parses test properly' do
    r = Enum1.from_file('src/enum_0.bin')

    expect(r.main.submain.pet_1).to eq :animal_cat
    expect(r.main.submain.pet_2).to eq :animal_chicken
  end
end

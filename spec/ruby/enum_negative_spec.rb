# coding: utf-8
require 'enum_negative'

RSpec.describe EnumNegative do
  it 'parses test properly' do
    r = EnumNegative.from_file('src/enum_negative.bin')

    expect(r.f1).to eq :constants_negative_one
    expect(r.f2).to eq :constants_positive_one
  end
end

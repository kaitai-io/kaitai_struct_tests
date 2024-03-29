# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ExprArray' do
  it 'parses test properly' do
    require 'expr_array'
    r = ExprArray.from_file('src/expr_array.bin')

    expect(r.aint_size).to eq 4
    expect(r.aint_first).to eq 7657765
    expect(r.aint_last).to eq 16272640
    expect(r.aint_min).to eq 49185
    expect(r.aint_max).to eq 1123362332
    expect(r.afloat_size).to eq 3
    expect(r.afloat_first).to be_within(1e-6).of -2.6839530254859364E-121
    expect(r.afloat_last).to be_within(1e-6).of -1.1103359815095273E-175
    expect(r.afloat_min).to be_within(1e-6).of -8.754689149998834E+288
    expect(r.afloat_max).to be_within(1e-6).of -1.1103359815095273E-175
    expect(r.astr_size).to eq 3
    expect(r.astr_first).to eq "foo"
    expect(r.astr_last).to eq "baz"
    expect(r.astr_min).to eq "bar"
    expect(r.astr_max).to eq "foo"
  end
end

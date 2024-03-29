# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ExprOpsParens' do
  it 'parses test properly' do
    require 'expr_ops_parens'
    r = ExprOpsParens.from_file('src/enum_negative.bin')

    expect(r.i_sum_to_str).to eq "29"
    expect(r.f_sum_to_int).to eq 9
    expect(r.str_concat_len).to eq 10
    expect(r.str_concat_rev).to eq "9876543210"
    expect(r.str_concat_substr_2_to_7).to eq "23456"
    expect(r.str_concat_to_i).to eq 123456789
    expect(r.bool_eq).to eq 0
    expect(r.bool_and).to eq 0
    expect(r.bool_or).to eq 1
  end
end

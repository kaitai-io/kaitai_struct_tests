# coding: utf-8
require 'expr_2'

RSpec.describe Expr2 do
  it 'parses test properly' do
    r = Expr2.from_file('src/str_encodings.bin')

    expect(r.str1.len_orig).to eq 10
    expect(r.str1.len_mod).to eq 7
    expect(r.str1.str).to eq "Some AS"

    expect(r.str1_len).to eq 7
    expect(r.str1_len_mod).to eq 7
    expect(r.str1_byte1).to eq 0x49
    expect(r.str1_avg).to eq 0x49
    expect(r.str1_char5).to eq "e"

    expect(r.str1_tuple5.byte0).to eq 0x65
    expect(r.str1_tuple5.byte1).to eq 0x20
    expect(r.str1_tuple5.byte2).to eq 0x41
    expect(r.str1_tuple5.avg).to eq 0x30

    expect(r.str2_tuple5.byte0).to eq 0x65
    expect(r.str2_tuple5.byte1).to eq 0x20
    expect(r.str2_tuple5.byte2).to eq 0x41
    expect(r.str2_tuple5.avg).to eq 0x30
  end
end

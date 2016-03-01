# coding: utf-8
require 'expr_0'

RSpec.describe Expr0 do
  it 'parses test properly' do
    r = Expr0.from_file('src/str_encodings.bin')

    expect(r.must_be_f7).to eq 0xf7
    expect(r.must_be_abc123).to eq 'abc123'
  end
end

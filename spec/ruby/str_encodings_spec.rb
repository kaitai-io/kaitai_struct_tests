# coding: utf-8
require 'str_encodings'

RSpec.describe Str_encodings do
  it 'parses test properly' do
    r = Str_encodings.from_file('src/str_encodings.bin')

    expect(r.str1).to eq 'Some ASCII'
    expect(r.str2.encode('UTF-8')).to eq 'こんにちは'
    expect(r.str3.encode('UTF-8')).to eq 'こんにちは'
    expect(r.str4.encode('UTF-8')).to eq '░▒▓'
  end
end

# coding: utf-8

RSpec.describe 'StrEncodings' do
  it 'parses test properly' do
    require 'str_encodings'
    r = StrEncodings.from_file('src/str_encodings.bin')

    expect(r.str1).to eq 'Some ASCII'
    expect(r.str2.encode('UTF-8')).to eq 'こんにちは'
    expect(r.str3.encode('UTF-8')).to eq 'こんにちは'
    expect(r.str4.encode('UTF-8')).to eq '░▒▓'
  end
end

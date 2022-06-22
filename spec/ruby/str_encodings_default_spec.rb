# coding: utf-8

RSpec.describe 'StrEncodingsDefault' do
  it 'parses test properly' do
    require 'str_encodings_default'
    r = StrEncodingsDefault.from_file('src/str_encodings.bin')

    expect(r.str1).to eq 'Some ASCII'
    expect(r.rest.str2.encode('UTF-8')).to eq 'こんにちは'
    expect(r.rest.str3.encode('UTF-8')).to eq 'こんにちは'
    expect(r.rest.str4.encode('UTF-8')).to eq '░▒▓'
  end
end

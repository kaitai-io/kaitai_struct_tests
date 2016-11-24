# coding: utf-8
require 'str_encodings_default'

RSpec.describe StrEncodingsDefault do
  it 'parses test properly' do
    r = StrEncodingsDefault.from_file('src/str_encodings.bin')

    expect(r.str1).to eq 'Some ASCII'
    expect(r.rest.str2.encode('UTF-8')).to eq 'こんにちは'
    expect(r.rest.str3.encode('UTF-8')).to eq 'こんにちは'
    expect(r.rest.str4.encode('UTF-8')).to eq '░▒▓'
  end
end

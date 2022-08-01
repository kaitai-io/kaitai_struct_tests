# coding: utf-8

RSpec.describe 'StrEncodingsUtf16' do
  it 'parses test properly' do
    require 'str_encodings_utf16'
    r = StrEncodingsUtf16.from_file('src/str_encodings_utf16.bin')

    expect(r.len_be).to eq 12
    expect(r.be_bom_removed.bom).to eq 65279
    expect(r.be_bom_removed.str.encode('UTF-8')).to eq 'こんにちは'
    expect(r.len_le).to eq 12
    expect(r.le_bom_removed.bom).to eq 65279
    expect(r.le_bom_removed.str.encode('UTF-8')).to eq 'こんにちは'
  end
end

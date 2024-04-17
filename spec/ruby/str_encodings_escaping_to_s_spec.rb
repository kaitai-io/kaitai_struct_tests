RSpec.describe 'StrEncodingsEscapingToS' do
  it 'parses test properly' do
    require 'str_encodings_escaping_to_s'
    r = StrEncodingsEscapingToS.from_file('src/str_encodings.bin')

    expect {
      r.str1
    }.to raise_error(ArgumentError, "unknown encoding name - ASCII\\\\x")
    expect {
      r.str2
    }.to raise_error(ArgumentError, "unknown encoding name - UTF-8\\'x")
    expect {
      r.str3
    }.to raise_error(ArgumentError, "unknown encoding name - SJIS\\\"x")
    expect {
      r.str4
    }.to raise_error(ArgumentError, "unknown encoding name - IBM437\\nx")
  end
end

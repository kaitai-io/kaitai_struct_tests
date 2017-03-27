require 'bytes_pad_term'

RSpec.describe BytesPadTerm do
  it 'parses test properly' do
    r = BytesPadTerm.from_file('src/str_pad_term.bin')

    expect(r.str_pad).to eq 'str1'
    expect(r.str_term).to eq 'str2foo'
    expect(r.str_term_and_pad).to eq 'str+++3bar+++'
    expect(r.str_term_include).to eq 'str4baz@'
  end
end

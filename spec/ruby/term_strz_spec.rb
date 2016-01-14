require 'term_strz'

RSpec.describe Term_strz do
  it 'parses test properly' do
    r = Term_strz.from_file('src/term_strz.bin')

    expect(r.s1).to eq('foo')
    expect(r.s2).to eq('bar')
    expect(r.s3).to eq('|baz@')
  end
end

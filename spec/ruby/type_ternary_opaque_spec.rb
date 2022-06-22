RSpec.describe 'TypeTernaryOpaque' do
  it 'parses test properly' do
    require 'type_ternary_opaque'
    r = TypeTernaryOpaque.from_file('src/term_strz.bin')

    expect(r.dif.s1).to eq('foo')
    expect(r.dif.s2).to eq('bar')
    expect(r.dif.s3).to eq('|baz@')
  end
end

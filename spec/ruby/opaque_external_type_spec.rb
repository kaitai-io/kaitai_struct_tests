RSpec.describe 'OpaqueExternalType' do
  it 'parses test properly' do
    require 'opaque_external_type'
    require 'term_strz' # the opaque type
    r = OpaqueExternalType.from_file('src/term_strz.bin')

    expect(r.one.s1).to eq('foo')
    expect(r.one.s2).to eq('bar')
    expect(r.one.s3).to eq('|baz@')
  end
end

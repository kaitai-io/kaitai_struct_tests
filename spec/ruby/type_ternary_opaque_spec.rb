RSpec.describe 'TypeTernaryOpaque' do
  it 'parses test properly' do
    require 'type_ternary_opaque'
    require 'hello_world' # the opaque type
    r = TypeTernaryOpaque.from_file('src/term_strz.bin')

    expect(r.dif.one).to eq 102
  end
end

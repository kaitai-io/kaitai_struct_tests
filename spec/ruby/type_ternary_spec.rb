require 'type_ternary'

RSpec.describe TypeTernary do
  it 'parses test properly' do
    r = TypeTernary.from_file('src/term_strz.bin')

    expect(r.dif.value).to eq 0x65
  end
end

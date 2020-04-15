require 'to_string_custom'

RSpec.describe ToStringCustom do
  it 'parses test properly' do
    r = ToStringCustom.from_file('src/term_strz.bin')

    expect(r.inspect).to eq 's1 = foo, s2 = bar'
  end
end

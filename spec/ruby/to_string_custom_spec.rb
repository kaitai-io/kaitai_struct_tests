RSpec.describe 'ToStringCustom' do
  it 'parses test properly' do
    require 'to_string_custom'
    r = ToStringCustom.from_file('src/term_strz.bin')

    expect("#{r}").to eq 's1 = foo, s2 = bar'
  end
end

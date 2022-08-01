RSpec.describe 'StrLiterals' do
  it 'parses test properly' do
    require 'str_literals'
    r = StrLiterals.from_file('src/fixed_struct.bin')

    expect(r.complex_str.chars.map { |x| x.ord }).to eq [0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787]
    expect(r.double_quotes.chars.map { |x| x.ord }).to eq [34, 34, 34]
    expect(r.backslashes.chars.map { |x| x.ord }).to eq [92, 92, 92]
    expect(r.octal_eatup.chars.map { |x| x.ord }).to eq [0, 50, 50]
    expect(r.octal_eatup2.chars.map { |x| x.ord }).to eq [2, 50]
  end
end

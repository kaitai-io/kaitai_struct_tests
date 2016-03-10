require 'default_big_endian'

RSpec.describe DefaultBigEndian do
  it 'parses test properly' do
    r = DefaultBigEndian.from_file('src/enum_0.bin')

    expect(r.one).to eq 0x7000000
  end
end

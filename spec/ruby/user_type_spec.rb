require 'user_type'

RSpec.describe UserType do
  it 'parses test properly' do
    r = UserType.from_file('src/repeat_until_s4.bin')

    expect(r.one.width).to eq 0x42
    expect(r.one.height).to eq 0x1337
  end
end

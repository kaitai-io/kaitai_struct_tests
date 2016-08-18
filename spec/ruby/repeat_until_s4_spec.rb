require 'repeat_until_s4'

RSpec.describe RepeatUntilS4 do
  it 'parses test properly' do
    r = RepeatUntilS4.from_file('src/repeat_until_s4.bin')

    expect(r.entries).to eq [0x42, 0x1337, -251658241, -1]
    expect(r.afterall).to eq "foobar"
  end
end

require 'repeat_eos_u4'

RSpec.describe RepeatEosU4 do
  it 'parses test properly' do
    r = RepeatEosU4.from_file('src/repeat_eos_struct.bin')

    expect(r.numbers).to eq([0, 0x42, 0x42, 0x815])
  end
end

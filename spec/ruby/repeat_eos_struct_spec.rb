require 'repeat_eos_struct'

RSpec.describe RepeatEosStruct do
  it 'parses test properly' do
    r = RepeatEosStruct.from_file('src/repeat_eos_struct.bin')

    expect(r.chunks.size).to eq(2)
    expect(r.chunks[0].offset).to eq 0
    expect(r.chunks[0].len).to eq 0x42
    expect(r.chunks[1].offset).to eq 0x42
    expect(r.chunks[1].len).to eq 0x815
  end
end

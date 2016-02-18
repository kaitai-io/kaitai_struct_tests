require 'repeat_n_struct'

RSpec.describe RepeatNStruct do
  it 'parses test properly' do
    r = RepeatNStruct.from_file('src/repeat_n_struct.bin')

    expect(r.chunks.size).to eq(2)
    expect(r.chunks[0].offset).to eq 0x10
    expect(r.chunks[0].len).to eq 0x2078
    expect(r.chunks[1].offset).to eq 0x2088
    expect(r.chunks[1].len).to eq 0xf
  end
end

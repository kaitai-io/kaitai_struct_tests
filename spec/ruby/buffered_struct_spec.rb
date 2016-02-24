require 'buffered_struct'

RSpec.describe BufferedStruct do
  it 'parses test properly' do
    r = BufferedStruct.from_file('src/buffered_struct.bin')

    expect(r.len1).to eq 0x10
    expect(r.block1.number1).to eq 0x42
    expect(r.block1.number2).to eq 0x43
    expect(r.len2).to eq 0x8
    expect(r.block2.number1).to eq 0x44
    expect(r.block2.number2).to eq 0x45
    expect(r.finisher).to eq 0xee
  end
end

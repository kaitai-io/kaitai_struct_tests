require 'bits_byte_aligned'

RSpec.describe BitsByteAligned do
  it 'parses test properly' do
    r = BitsByteAligned.from_file('src/fixed_struct.bin')

    # 50 (6 + 2) = 010100|00
    expect(r.one).to eq 0b010100
    # 41
    expect(r.byte_1).to eq 0x41
    # 43 (3 + 1 + 4) = 010|0|0011
    expect(r.two).to eq 0b010
    expect(r.three).to eq false
    # 4B
    expect(r.byte_2).to eq 0x4b
    # 2D 31 (14 + 2) = 00101101 001100|01
    expect(r.four).to eq 0b00101101_001100
    # FF
    expect(r.byte_3).to eq [0xff].pack('C*')
    # FF
    expect(r.full_byte).to eq 0xff
    # 50
    expect(r.byte_4).to eq 0x50
  end
end

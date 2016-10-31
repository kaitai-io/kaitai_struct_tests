require 'expr_io_pos'

RSpec.describe ExprIoPos do
  it 'parses test properly' do
    r = ExprIoPos.from_file('src/expr_io_pos.bin')

    expect(r.substream1.my_str).to eq 'CURIOSITY'
    expect(r.substream1.body).to eq [0x11, 0x22, 0x33, 0x44].pack('C*')
    expect(r.substream1.number).to eq 0x42

    expect(r.substream2.my_str).to eq 'KILLED'
    expect(r.substream2.body).to eq [0x61, 0x20, 0x63, 0x61, 0x74].pack('C*')
    expect(r.substream2.number).to eq 0x67
  end
end

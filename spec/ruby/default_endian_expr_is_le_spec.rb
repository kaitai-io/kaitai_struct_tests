require 'default_endian_expr_is_le'

RSpec.describe DefaultEndianExprIsLe do
  it 'parses test properly' do
    r = DefaultEndianExprIsLe.from_file('src/endian_expr.bin')

    expect(r.docs[0].indicator).to eq [0x49, 0x49].pack('C*')
    expect(r.docs[0].main.some_int).to eq 0x42
    expect(r.docs[0].main.some_int_be).to eq 0x42
    expect(r.docs[0].main.some_int_le).to eq 0x42

    expect(r.docs[1].indicator).to eq [0x4d, 0x4d].pack('C*')
    expect(r.docs[1].main.some_int).to eq 0x42
    expect(r.docs[1].main.some_int_be).to eq 0x42
    expect(r.docs[1].main.some_int_le).to eq 0x42

    expect(r.docs[2].indicator).to eq [0x58, 0x58].pack('C*')
    expect(r.docs[2].main.some_int).to eq 0x42
    expect(r.docs[2].main.some_int_be).to eq 0x42
    expect(r.docs[2].main.some_int_le).to eq 0x42
  end
end

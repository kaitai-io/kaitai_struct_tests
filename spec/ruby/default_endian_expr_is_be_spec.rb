require 'default_endian_expr_is_be'

RSpec.describe DefaultEndianExprIsBe do
  it 'parses test properly' do
    r = DefaultEndianExprIsBe.from_file('src/endian_expr.bin')

    # LE
    expect(r.docs[0].indicator).to eq [0x49, 0x49].pack('C*')
    expect(r.docs[0].main.some_int).to eq 0x42
    expect(r.docs[0].main.some_int_be).to eq 0x42
    expect(r.docs[0].main.some_int_le).to eq 0x42

    expect(r.docs[0].main.inst_int).to eq 0x42
    expect(r.docs[0].main.inst_sub.foo).to eq 0x42

    # BE
    expect(r.docs[1].indicator).to eq [0x4d, 0x4d].pack('C*')
    expect(r.docs[1].main.some_int).to eq 0x42
    expect(r.docs[1].main.some_int_be).to eq 0x42
    expect(r.docs[1].main.some_int_le).to eq 0x42

    expect(r.docs[1].main.inst_int).to eq 0x42000000
    expect(r.docs[1].main.inst_sub.foo).to eq 0x42000000

    # Weird => LE
    expect(r.docs[2].indicator).to eq [0x58, 0x58].pack('C*')
    expect(r.docs[2].main.some_int).to eq 0x42000000
    expect(r.docs[2].main.some_int_be).to eq 0x42
    expect(r.docs[2].main.some_int_le).to eq 0x42

    expect(r.docs[2].main.inst_int).to eq 0x42
    expect(r.docs[2].main.inst_sub.foo).to eq 0x42
  end
end

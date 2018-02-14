require 'default_endian_expr_inherited'

RSpec.describe DefaultEndianExprInherited do
  it 'parses test properly' do
    r = DefaultEndianExprInherited.from_file('src/endian_expr.bin')

    expect(r.docs[0].indicator).to eq [0x49, 0x49].pack('C*')
    expect(r.docs[0].main.insides.some_int).to eq 0x42
    expect(r.docs[0].main.insides.more.some_int1).to eq 0x4200
    expect(r.docs[0].main.insides.more.some_int2).to eq 0x42
    expect(r.docs[0].main.insides.more.some_inst).to eq 0x42

    expect(r.docs[1].indicator).to eq [0x4d, 0x4d].pack('C*')
    expect(r.docs[1].main.insides.some_int).to eq 0x42
    expect(r.docs[1].main.insides.more.some_int1).to eq 0x42
    expect(r.docs[1].main.insides.more.some_int2).to eq 0x4200
    expect(r.docs[1].main.insides.more.some_inst).to eq 0x42000000

    expect(r.docs[2].indicator).to eq [0x58, 0x58].pack('C*')
    expect(r.docs[2].main.insides.some_int).to eq 0x42
    expect(r.docs[2].main.insides.more.some_int1).to eq 0x42
    expect(r.docs[2].main.insides.more.some_int2).to eq 0x4200
    expect(r.docs[2].main.insides.more.some_inst).to eq 0x42000000
  end
end

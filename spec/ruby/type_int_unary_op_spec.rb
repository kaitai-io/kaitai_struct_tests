require 'type_int_unary_op'

RSpec.describe TypeIntUnaryOp do
  it 'parses test properly' do
    r = TypeIntUnaryOp.from_file('src/fixed_struct.bin')

    expect(r.value_s2).to eq(0x4150)
    expect(r.value_s8).to eq(0x4150ffff312d4b43)
    expect(r.unary_s2).to eq(-0x4150)
    expect(r.unary_s8).to eq(-0x4150ffff312d4b43)
  end
end

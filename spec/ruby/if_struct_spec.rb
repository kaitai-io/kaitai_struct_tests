require 'if_struct'

RSpec.describe IfStruct do
  it 'parses test properly' do
    r = IfStruct.from_file('src/if_struct.bin')

    expect(r.op1.opcode).to eq(0x53)
    expect(r.op1.arg_str.str).to eq('foo')

    expect(r.op2.opcode).to eq(0x54)
    expect(r.op2.arg_tuple.num1).to eq(0x42)
    expect(r.op2.arg_tuple.num2).to eq(0x43)

    expect(r.op3.opcode).to eq(0x53)
    expect(r.op3.arg_str.str).to eq('bar')
  end
end

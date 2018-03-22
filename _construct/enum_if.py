def opcodes(subcon):
	return Enum(subcon, a_string=0x53, a_tuple=0x54)

arg_tuple = Struct(
	'num1' / Int8ul,
	'num2' / Int8ul,
)

arg_str = Struct(
	'len' / Int8ul,
	'str' / FixedSized(this.len, GreedyString(encoding='utf8')),
)

operation = Struct(
	'opcode' / opcodes(Int8ul),
	'arg_tuple' / If(this.opcode == 'a_tuple', arg_tuple),
	'arg_str' / If(this.opcode == 'a_string', arg_str),
)

Struct(
	'op1' / operation,
	'op2' / operation,
	'op3' / operation,
)

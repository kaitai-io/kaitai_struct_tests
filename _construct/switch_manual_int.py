intval = Struct(
	'value' / Int8ul,
)

strval = Struct(
	'value' / CString(encoding='ascii'),
)

opcode = Struct(
	'code' / Int8ul,
	'body' / Switch(this.code, {
		73: intval,
		83: strval,
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

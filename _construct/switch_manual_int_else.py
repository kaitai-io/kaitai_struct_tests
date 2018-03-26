intval = Struct(
	'value' / Int8ul,
)

strval = Struct(
	'value' / CString(encoding='ascii'),
)

noneval = Struct(
	'filler' / Int32ul,
)

opcode = Struct(
	'code' / Int8ul,
	'body' / Switch(this.code, {
		73: intval,
		83: strval,
	}, default=noneval),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

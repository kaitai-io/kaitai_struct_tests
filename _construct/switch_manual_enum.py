intval = Struct(
	'value' / Int8ul,
)

strval = Struct(
	'value' / CString(encoding='ascii'),
)

opcode = Struct(
	'code' / Enum(Int8ul, intval=73, strval=83),
	'body' / Switch(this.code, {
		'intval': intval,
		'strval': strval,
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

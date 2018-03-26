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
	'code' / FixedSized(1, GreedyString(encoding='ascii')),
	'body' / Switch(this.code, {
		u'I': intval,
		u'S': strval,
	}, default=noneval),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

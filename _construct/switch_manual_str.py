intval = Struct(
	'value' / Int8ul,
)

strval = Struct(
	'value' / CString(encoding='ascii'),
)

opcode = Struct(
	'code' / FixedSized(1, GreedyString(encoding='ascii')),
	'code' / StringEncoded(Bytes(1)),
	# alternative, faster because it doesnt use substreams
	# but it has different build semantics (takes a bytes of exact length instead of adding variable padding)
	# therefore I propose to use FixedSized and this one be optional as -construct-render: StringEncodedBytes

	'body' / Switch(this.code, {
		u'I': intval,
		u'S': strval,
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

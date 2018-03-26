intval = Struct(
	'value' / Int8ul,
)

strval = Struct(
	'value' / CString(encoding='ascii'),
)

opcode = Struct(
	'code' / Bytes(1),
	'body' / Switch(this.code, {
		b'\x49': intval,
		b'\x53': strval,
		# if -construct-render: StructValue inside intval and strval, then perhaps it should be:
		# b'\x73': Int8ul,
		# b'\x83': CString(encoding='ascii'),
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

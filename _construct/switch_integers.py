opcode = Struct(
	'code' / Int8ul,
	'body' / Switch(this.code, {
		# alternatively BytesInteger(1 or 2 or 4 or 8)
		1: Int8ul,
		2: Int16ul,
		4: Int32ul,
		8: Int64ul,
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

opcode = Struct(
	'code' / Int8ul,
	'body' / Switch(lambda this: this.code if ((this.code > 0) and (this.code <= 8) and (True if (this.code != 10) else False)) else 0, {
		1: Int8ul,
		2: Int16ul,
		4: Int32ul,
		8: Int64ul,
	}),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

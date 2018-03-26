one = Struct(
	'first' / GreedyBytes,
)

two = Struct(
	'second' / GreedyBytes,
)

opcode = Struct(
	'code' / Int8ul,
	'size' / Int32ul,
	'body' / Array(1, FixedSized(this.size, Switch(this.code, {
		0x11: one,
        0x22: two,
	}))),
)

Struct(
	'opcodes' / GreedyRange(opcode),
)

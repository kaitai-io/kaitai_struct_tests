code = Struct(
	'opcode' / Int8ul,
	'half_opcode' / If(this.opcode % 2 == 0, Computed(this.opcode // 2)),
)

Struct(
	'codes' / Array(3, code),
)

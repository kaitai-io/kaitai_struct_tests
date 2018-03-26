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
	# the .as<strval> has no meaning in Construct, the body is a string or is an integer
	# (it is what it was parsed as), how could this be reinterpreted?
	'first_obj' / Lazy(Computed(lambda this: this.opcodes[0])),
	'second_val' / Lazy(Computed(lambda this: this.opcodes[1])),
	'err_cast' / Lazy(Computed(lambda this: this.opcodes[2])),
)

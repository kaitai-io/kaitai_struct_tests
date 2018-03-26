Struct(
	'code' / Int8ul,
	'len' / Switch(this.code, {
		# alternatively BytesInteger(1 or 2 or 4 or 8)
		1: Int8ul,
		2: Int16ul,
		4: Int32ul,
		8: Int64ul,
	}),
	'ham' / Bytes(this.len),
	'padding' / If(this.len > 3, Int8ul),
	# perhaps instances should be added on top instead so they can be used within other fields
	# also Lazy is not yet implemented at the time of writing
	'len_mod_str' / Lazy(Computed(lambda this: str(this.len * 2 - 1))),
)

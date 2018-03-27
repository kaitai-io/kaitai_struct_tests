Struct(
	'len_of_1' / Int16ul,
	'must_be_f7' / Computed(7 + 0xf0),
	# are those unicode or byte strings?
	'must_be_abc123' / Computed("abc" + "123"),
)

Struct(
	'len_of_1' / Int16ul,
	'str1' / FixedSized(this.len_of_1, GreedyString(encoding='ascii')),
	'len_of_1_mod' / Computed(this.len_of_1 - 2),
	'str1_len' / Computed(lambda this: len(this.str1)),
)

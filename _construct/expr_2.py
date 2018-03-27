tuple = Struct(
	# instances
	# float division / or integer division // ?
	'avg' / Lazy(Computed((lambda this: this.byte1 + this.byte2) / 2)),
	# seq
	'byte0' / Int8ul,
	'byte1' / Int8ul,
	'byte2' / Int8ul,
)

mod_str = Struct(
	# instances are pre-appended so they can be referenced by seq fields
	'len_mod' / Lazy(Computed(lambda this: this.len_orig - 3)),
	'char5' / Lazy(Pointer(5, FixedSized(1, GreedyString(encoding='ascii')))),
	'tuple5' / Lazy(Pointer(5, tuple)),
	# seq fields
	'len_orig' / Int16ul,
	'str' / FixedSized(this.len_mod, GreedyString(encoding='utf-8')),
	'rest' / FixedSized(3, tuple),
)

Struct(
	# instances
	'str1_len' / Lazy(Computed(lambda this: len(this.str1.str))),
	'str1_len_mod' / Lazy(Computed(lambda this: this.str1.len_mod)),
	'str1_byte1' / Lazy(Computed(lambda this: this.str1.rest.byte1)),
	'str1_avg' / Lazy(Computed(lambda this: this.str1.rest.avg)),
	'str1_byte1' / Lazy(Computed(lambda this: str1.rest.byte1)),
	'str1_char5' / Lazy(Computed(lambda this: str1.char5)),
	'str1_tuple5' / Lazy(Computed(lambda this: str1.tuple5)),
	'str2_tuple5' / Lazy(Computed(lambda this: str2.tuple5)),
	# seq fields
	'str1' / mod_str,
	'str2' / mod_str,
)

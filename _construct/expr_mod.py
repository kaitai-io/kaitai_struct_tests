Struct(
	'int_u' / Int32ul,
	'int_s' / Int32sl,
	'mod_pos_const' / Computed(9837 % 13),
	'mod_neg_const' / Computed(-9837 % 13),
	'mod_pos_seq' / Computed(this.int_u % 13),
	'mod_neg_seq' / Computed(this.int_s % 13),
)

Struct(
	'one' / Int8ul,
	'two' / FixedSized(3, GreedyString(encoding='ascii')),
	# instances
	'three' / Computed(u"@" + this.two),
	'four' / Computed(u"_" + this.two + u"_"),
	'is_str_eq' / Computed(this.two == u"ACK"),
	'is_str_ne' / Computed(this.two != u"ACK"),
	'is_str_lt' / Computed(this.two < u"ACK"),
	'is_str_gt' / Computed(this.two > u"ACK"),
	'is_str_le' / Computed(this.two <= u"ACK"),
	'is_str_ge' / Computed(this.two >= u"ACK"),
	'is_str_lt2' / Computed(this.three < this.two),
	# a literal "True" would not be a problem but the "not" is not supported by this expressions, it must be lambdafied
	'test_not' / Computed(lambda this: not False),
)

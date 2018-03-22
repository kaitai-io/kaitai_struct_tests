Struct(
	'len1' / Int32ul,
	'block1' / FixedSized(this.len1, Struct(
		'number1' / Int32ul,
		'number2' / Int32ul,
	)),
	'len2' / Int32ul,
	'block2' / FixedSized(this.len2, Struct(
		'number1' / Int32ul,
		'number2' / Int32ul,
	)),
	'finisher' / Int32ul,
)

# this could be refactored
block = Struct(
	'number1' / Int32ul,
	'number2' / Int32ul,
)
Struct(
	'len1' / Int32ul,
	'block1' / FixedSized(this.len1, block),
	'len2' / Int32ul,
	'block2' / FixedSized(this.len2, block),
	'finisher' / Int32ul,
)

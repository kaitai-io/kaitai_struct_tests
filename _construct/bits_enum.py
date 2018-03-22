BitStruct(
	# could use macro like in other examples
	'one' / Enum(BitsInteger(4), cat=0, dog=1, horse=4, platypus=5),
	'two' / Enum(BitsInteger(8), cat=0, dog=1, horse=4, platypus=5),
	'three' / Enum(BitsInteger(1), cat=0, dog=1, horse=4, platypus=5),
	# must sum up to multiple of 8 or fails during parsing
	Padding(3),
)

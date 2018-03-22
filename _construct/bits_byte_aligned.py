# note its Bitwise(Struct) and not Struct
# that is why byte-oriented classes are wrapped in Bytewise
# I suggest a heuristic, if contains bit-integers then its BitStruct.
# But a bitstruct cannot contain other things like strings (byte-oriented stuff)
BitStruct(
	'one' / BitsInteger(6),
	# unnamed fields are discarded (not appear in result dict)
	Padding(2),
	'byte_1' / BitsInteger(8) or Bytewise(Int8ub),
	'two' / BitsInteger(3),
	# Flag is also single-bit but boolean, not integer
	# I make a proposal to add a tag for that purpose.
	# -construct-render: Flag
	'three' / BitsInteger(1),
	Padding(4),
	'byte_2' / BitsInteger(8) or Bytewise(Int8ub),
	'four' / BitsInteger(14),
	Padding(2),
	'byte_3' / Bytwise(Bytes(1)),
	'full_byte' / BitsInteger(8),
	'byte_4' / BitsInteger(8) or Bytewise(Int8ub),
)

# less attractive but valid analog would be
Struct(
	'one' / BitStruct('one'/BitsInteger(6), Padding(2), )
	'byte_1' / Int8ub,
	'two' / BitStruct('two'/BitsInteger(3), 'three'/BitsInteger(1), Padding(4), )
	'byte_2' / Int8ub,
	'four' / BitStruct('four'/BitsInteger(14), Padding(2), )
	'byte_3' / Bytes(1),
	'full_byte' / BitStruct('full_byte'/BitsInteger(8), )
	'byte_4' / Int8ub,
)

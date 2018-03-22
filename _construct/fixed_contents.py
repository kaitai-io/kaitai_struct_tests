Struct(
	'normal' / Const(b'PACK-1'),
	# bytes would be better for performance, but array is readable
	'high_bit_8' / Const([0xff, 0xff], Array(2, Byte)),
)

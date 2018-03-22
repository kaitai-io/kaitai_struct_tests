BitStruct(
	'byte_1' / BitsInteger(8),
	'byte_2' / BitsInteger(8),
	'bits_a' / BitsInteger(1),
	'bits_b' / BitsInteger(3),
	'bits_c' / BitsInteger(4),
	'large_bits_1' / BitsInteger(10),
	'spacer' / BitsInteger(3),
	'large_bits_2' / BitsInteger(11),
	'normal_s2' / BitsInteger(16, signed=True, swapped=False),
	'byte_8_9_10' / BitsInteger(24),
	'byte_11_to_14' / BitsInteger(32),
	'byte_15_to_19' / BitsInteger(40),
	'byte_20_to_27' / BitsInteger(64),
	# ordering barrier
	# cannot equal False because its integer not boolean
	'test_if_b1' / If(this.bits_a == 0, Computed(123)),
)

Struct(
	'key' / Bytes(4),
	'buf' / ProcessXor(b'\xec\xbb\xa3\x14', GreedyBytes),
)

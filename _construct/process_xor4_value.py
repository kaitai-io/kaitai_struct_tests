Struct(
	'key' / Bytes(4),
	'buf' / ProcessXor(this.key, GreedyBytes),
)

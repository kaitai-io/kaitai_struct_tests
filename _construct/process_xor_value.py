Struct(
	'key' / Int8ul,
	'buf' / ProcessXor(this.key, GreedyBytes),
)

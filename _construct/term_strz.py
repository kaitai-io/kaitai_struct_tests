Struct(
	's1' / NullTerminated(GreedyString(encoding='utf-8'), term=b'\x7c'),
	's2' / NullTerminated(GreedyString(encoding='utf-8'), term=b'\x7c', consume=False),
	's3' / NullTerminated(GreedyString(encoding='utf-8'), term=b'\x40', include=True),
)

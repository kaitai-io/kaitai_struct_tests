# the default parameters
# NullTerminated(subcon, term=b'\x00', include=False, consume=True, require=True)
# NullStripped(subcon, pad=b'\x00')

Struct(
	's1' / NullTerminated(GreedyBytes, term=b'\x7c'),
	's2' / NullTerminated(GreedyBytes, term=b'\x7c', consume=False),
	's3' / NullTerminated(GreedyBytes, term=b'\x40', include=True),
)

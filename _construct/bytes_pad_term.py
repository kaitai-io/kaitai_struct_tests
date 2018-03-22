Struct(
	'str_pad' / FixedSized(20, NullStripped( GreedyBytes, pad=b'\x2b')),
	'str_term' / FixedSized(20, NullTerminated( GreedyBytes, term=b'\x40')),
	'str_term_and_pad' / FixedSized(20, NullStripped( NullTerminated( GreedyBytes, term=b'\x40'), pad=b'\x2b')),
	'str_term_include' / FixedSized(20, NullTerminated( GreedyBytes, term=b'\x40', include=True)),
)

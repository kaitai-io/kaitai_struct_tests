one_or_two = Struct(
	'one' / Int32ul,
	'two' / If(lambda this: not this._stream.eof, Int32ul),
	# instances
	'reflect_eof' / Lazy(Computed(lambda this: this._stream.eof)),
)

Struct(
	'substream1' / FixedSized(4, one_or_two),
	'substream2' / FixedSized(8, one_or_two),
)

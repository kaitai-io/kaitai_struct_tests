all_plus_number = Struct(
	'my_str' / CString('utf-8'),
	'body' / FixedSized(lambda this: this._stream.size - this._stream.offset - 2),
	'number' / Int16ul,
)

Struct(
	'substream1' / FixedSized(16, all_plus_number),
	'substream2' / FixedSized(14, all_plus_number),
)

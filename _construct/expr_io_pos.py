all_plus_number = Struct(
	'my_str' / CString('utf-8'),
	# ISSUE: Fails for same reason like other cases, lambda this and this._io
	'body' / FixedSized(lambda this: this._stream.size - this._stream.offset - 2),
	'number' / Int16ul,
)

Struct(
	'substream1' / FixedSized(16, all_plus_number),
	'substream2' / FixedSized(14, all_plus_number),
)

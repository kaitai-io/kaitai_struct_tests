chunk_meta = Struct(
	'title' / CString(encoding='utf-8'),
	'author' / CString(encoding='utf-8'),
)

chunk_dir = Struct(
	'entries' / GreedyRange(FixedSized(4, GreedyString(encoding='utf-8')))
)

chunk_body = Struct(
	'body' / GreedyRange(Switch(this._.code, {
		0x11: chunk_meta,
		0x22: chunk_dir,
	})),
)

chunk = Struct(
	'code' / Int8ul,
	'size' / Int32ul,
	'body' / FixedSized(this.size, chunk_body),
)

Struct(
	'chunks' / GreedyRange(chunk),
)

# ProcessRotateLeft is not yet implemented

Struct(
	'buf1' / ProcessRotateLeft(amount=3, group=1, Bytes(5)),
	'buf2' / ProcessRotateLeft(amount=-3, group=1, Bytes(5)),
	'key' / Int8ul,
	'buf3' / ProcessRotateLeft(amount=key, group=1, Bytes(5)),
)

record = Struct(
	'marker' / Int8ul,
	'body' / Int32ul,
)

Struct(
	# is this supposed to be 5 bytes total?
	'records' / FixedSized(5, RepeatUntil(lambda obj_,list_,this: obj_.marker == 0xaa, record)),
	# or 5 bytes per record?
	'records' / RepeatUntil(lambda obj_,list_,this: obj_.marker == 0xaa, FixedSized(5, record)),
)

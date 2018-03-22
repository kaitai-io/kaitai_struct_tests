Struct(
	'qty' / Int32ul,
	# you can use NullTerminated(GreedyString(encoding='utf-8'), term=bytes(1)) instead
	# for UTF16 it would be term=bytes(2) etc
	'lines' / Array(this.qty, CString(encoding='utf-8')),
)

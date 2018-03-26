Struct(
	'entries' / RepeatUntil(lambda obj_,list_,this: obj_ == -1, Int32sl),
	# alternatives
	# NullTerminated is valid for ascii/utf8 but will fail with utf16/utf32
	# also CString has better performance
	'afterall' / CString(encoding='ascii'),
	'afterall' / NullTerminated(GreedyString(encoding='ascii'))
)

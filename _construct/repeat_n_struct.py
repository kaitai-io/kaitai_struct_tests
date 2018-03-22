Struct(
	'qty' / Int32ul,
	'chunks' / Array(this.qty, Struct(
		'offset' / Int32ul,
		'len' / Int32ul,
	)),
)

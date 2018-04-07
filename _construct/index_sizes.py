
index_sizes = Struct(
	'qty' / Int32ul,
	'sizes' / Array(this.qty, Int32ul),
	# ISSUE: corrected below
	'bufs' / Array(this.qty, FixedSized(lambda this: this.sizes[this._index], GreedyString(encoding='ASCII'))),
	'bufs' / Array(this.qty, FixedSized(this.sizes[i], GreedyString(encoding='ASCII'))),
)

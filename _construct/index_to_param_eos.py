
index_to_param_eos__block = Struct(
	# ISSUE: corrected
	'buf' / FixedSized(lambda this: this._root.sizes[this._index], GreedyString(encoding='ASCII')),
	'buf' / FixedSized(this._root.sizes[this.idx], GreedyString(encoding='ASCII')),
)

index_to_param_eos = Struct(
	'qty' / Int32ul,
	'sizes' / Array(this.qty, Int32ul),
	'blocks' / GreedyRange(LazyBound(lambda: index_to_param_eos__block)),
)

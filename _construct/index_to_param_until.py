
index_to_param_until__block = Struct(
	# ISSUE: Its this._._index (not this._index) because the _index is maintained by Array GreedyRange RepeatUntil classes, but Struct then does context nesting. I will work around that on Construct side, so you can access it directly by this._index.
	# And entire expression needs to be 'lambda this:'
	# corrected:
	'buf' / FixedSized(lambda this: this._root.sizes[this._index], GreedyString(encoding='ASCII')),
	'buf' / FixedSized(this._root.sizes[this.idx], GreedyString(encoding='ASCII')),
)

index_to_param_until = Struct(
	'qty' / Int32ul,
	'sizes' / Array(this.qty, Int32ul),
	# ISSUE: Its this._io
	'blocks' / RepeatUntil(lambda obj_, list_, this: stream_iseof(_io), LazyBound(lambda: index_to_param_until__block)),
)

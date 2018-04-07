
instance_io_user__entry = Struct(
	'name_ofs' / Int32ul,
	'value' / Int32ul,
	# ISSUE: streamfunc is yet not implemented
	'name' / Pointer(this.name_ofs, NullTerminated(GreedyString(encoding='UTF-8'), term=b'\x00', include=False, consume=True), streamfunc=this._root.strings._io),
)

instance_io_user__strings_obj = Struct(
	'str' / GreedyRange(NullTerminated(GreedyString(encoding='UTF-8'), term=b'\x00', include=False, consume=True)),
)

instance_io_user = Struct(
	'qty_entries' / Int32ul,
	'entries' / Array(this.qty_entries, LazyBound(lambda: instance_io_user__entry)),
	# ISSUE: currently untranslatable
	# I would suggest ignoring the 'strings' size-eos, since the struct is already greedy
	'strings' / instance_io_user__strings_obj,
)

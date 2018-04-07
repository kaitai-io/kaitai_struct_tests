from construct import *
from construct.lib import *

position_to_end__index_obj = Struct(
	'foo' / Int32ul,
	'bar' / Int32ul,
)

position_to_end = Struct(
	# ISSUE: Pointer offset needs to be lambdafied (+ lambda this: )
	# Also its this._io
	'index' / Pointer((stream_size(_io) - 8), LazyBound(lambda: position_to_end__index_obj)),
)

_schema = position_to_end

from construct import *
from construct.lib import *

position_in_seq__header_obj = Struct(
	'qty_numbers' / Int32ul,
)

position_in_seq = Struct(
	# ISSUE: integer wrongly translated as big-endian
	'numbers' / Array(this.header.qty_numbers, Int8ub),

	# ISSUE: the instances should be BEFORE seq fields, they cannot be referenced by other fields that are above it
	# Also, *perhaps* instances should be wrapped in Lazy() although that only matters in some special cases
	'header' / Pointer(16, LazyBound(lambda: position_in_seq__header_obj)),
)

_schema = position_in_seq

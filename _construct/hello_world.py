from construct import *
from construct.lib import *

hello_world = Struct(
	'one' / Int8ub * "field doc tag",
) * \
"""
Schema name: hello_world (informative only)

This is equivalent of meta title tag.
Every construct can be multiplied with a string.

For convenience, every struct member should end with comma (allows reordering).
"""

_schema = hello_world

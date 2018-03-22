# docs and docrefs can be merged into multiline strings
# those strings are attached to fields the same way
Struct(
	'one' / Int8ub * 'Plain text description of doc ref, page 42',
	'two' / Int8ub * """
Both doc and doc-ref are defined
http://www.example.com/with/url/again
	""",
	'three' / Int8ub * 'http://www.example.com/three Documentation name',
	'foo' / Computed(True) * 'Doc ref for instance, a plain one',
	'parse_inst' / Pointer(0, Int8ub) * """
Now this is a really
long document ref that
spans multiple lines.
	""",
) * \
"""
Another one-liner
http://www.example.com/some/path/?even_with=query&more=2
"""

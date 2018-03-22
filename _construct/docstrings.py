Struct(
	'one' / Int32ul * 'A pretty verbose description for sequence attribute "one"',
	'complex_subtype' / Pass * """
This subtype is never used, yet has a very long description
that spans multiple lines. It should be formatted accordingly,
even in languages that have single-line comments for
docstrings. Actually, there's even a MarkDown-style list here
with several bullets:

* one
* two
* three

And the text continues after that. Here's a MarkDown-style link:
[woohoo](http://example.com) - one day it will be supported as
well.
	""",
	'two' / Pointer(0, Int32ul) * 'Another description for parse instance "two"',
	'three' / Computed(0x42) * 'And yet another one for value instance "three"',

) * \
"One-liner description of a type."

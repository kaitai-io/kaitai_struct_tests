def constants(subcon):
	return Enum(subcon, negative_one=-1, positive_one=1)

Struct(
	'f1' / constants(Int8sb),
	'f2' / constants(Int8sb),
)

subtype_b = Struct(
	'value_b' / Int8sb,
)

subtype_d = Struct(
	'value_d' / Int8sb,
)

subtype_cc = Struct(
	'value_cc' / Int8sb,
)

subtype_c = Struct(
	'value_c' / Int8sb,
	'typed_here' / subtype_d,
	'typed_parent' / subtype_cc,
	'typed_root' / subtype_b,
)

subtype_a = Struct(
	'typed_at_root' / subtype_b,
	'typed_here1' / subtype_c,
	'typed_here2' / subtype_cc,
)

Struct(
	'one' / subtype_a,
	'two' / subtype_b,
)

Struct(
	'one' / Struct(
		'typed_at_root' / Struct(
			'value_b' / Int8sb,
		),
		'typed_here' / Struct(
			'value_c' / Int8sb,
		),
	),
	'two' / Struct(
		'value_b' / Int8sb,
	),
)

# it is possible to reuse instances
subtype_b = Struct(
	'value_b' / Int8sb,
),
subtype_c, = Struct(
	'value_c' / Int8sb,
),
Struct(
	'one' / Struct(
		'typed_at_root' / subtype_b,
		'typed_here' / subtype_c,
	),
	'two' / subtype_b,
)

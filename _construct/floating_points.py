Struct(
	'single_value' / Float32l,
	'double_value' / Float64l,
	'single_value_be' / Float32b,
	'double_value_be' / Float64b,
	'approximate_value' / Float32l,
	# barrier, computed instances must follow the referred fields
	'single_value_plus_int' / Computed(this.single_value + 1),
	'single_value_plus_float' / Computed(this.single_value + 0.5),
	'double_value_plus_float' / Computed(this.double_value + 0.05),
)

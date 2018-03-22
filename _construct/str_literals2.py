# not sure if those are b- u- or unprefixed strings
# Python has recently gained string interpolation
Struct(
	'dollar1' / Computed('"$foo"'),
	'dollar2' / Computed('"${foo}"'),
	'hash' / Computed('"#{foo}"'),
	'at_sign' / Computed('"@foo"'),
)

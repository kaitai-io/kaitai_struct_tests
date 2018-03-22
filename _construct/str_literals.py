# not sure if those are b- u- or unprefixed strings
Struct(
	'complex_str' / Computed('"\0\1\2\a\b\n\r\t\v\f\e\75\7\u000a\u0024\u263b"'),
	'double_quotes' / Computed('"\"\u0022\42"'),
	'backslashes' / Computed('"\\\u005c\134"'),
	'octal_eatup' / Computed('"\0\62\62"'),
	'octal_eatup2' / Computed('"\2\62"'),
)

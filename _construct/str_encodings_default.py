Struct(
	'len_of_1' / Int16ul,
	'str1' / FixedSized(this.len_of_1, GreedyString(encoding='utf-8')),
	'rest' / Struct(
		'len_of_2' / Int16ul,
		'str2' / FixedSized(this.len_of_1, GreedyString(encoding='utf-8')),
		'len_of_3' / Int16ul,
		'str3' / FixedSized(this.len_of_1, GreedyString(encoding='SJIS')),
		'len_of_4' / Int16ul,
		'str4' / FixedSized(this.len_of_1, GreedyString(encoding='CP437')),
	),
)

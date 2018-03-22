# encodings can be lowercase or uppercase, doesnt matter
# also - and _ are interchangeable
Struct(
	'len_of_1' / Int16ul,
	'str1' / FixedSized(this.len_of_1, GreedyString(encoding='ascii')),
	'len_of_2' / Int16ul,
	'str2' / FixedSized(this.len_of_2, GreedyString(encoding='utf-8')),
	'len_of_3' / Int16ul,
	'str3' / FixedSized(this.len_of_3, GreedyString(encoding='SJIS')),
	'len_of_4' / Int16ul,
	'str4' / FixedSized(this.len_of_4, GreedyString(encoding='CP437')),
)

# it would be possible to use PascalString but...
# (1) it would hide lenght fields from results
# (2) compiler would need to guess/check if those fields are used otherwise

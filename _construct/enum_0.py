def animal(subcon):
	return Enum(subcon, dog=4, cat=7, chicken=12)

Struct(
	'pet_1' / animal(Int32ul),
	'pet_2' / animal(Int32ul),
)

# alternatively, if you dont like a macro
Struct(
	'pet_1' / Enum(Int32ul, dog=4, cat=7, chicken=12),
	'pet_2' / Enum(Int32ul, dog=4, cat=7, chicken=12),
)

# for reference
def enumwithoutlabels(subcon):
	return Enum(subcon, )

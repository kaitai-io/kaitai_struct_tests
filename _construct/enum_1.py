def animal(subcon):
	return Enum(subcon, dog=4, cat=7, chicken=12)

submain_obj = Struct(
	'pet_1' / animal(Int32ul),
	'pet_2' / animal(Int32ul),
)

main_obj = Struct(
	'submain' / submain_obj,
)

Struct(
	'main' / main_obj,
)

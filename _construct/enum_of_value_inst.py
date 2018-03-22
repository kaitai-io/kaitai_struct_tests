def animal(subcon):
	return Enum(subcon, dog=4, cat=7, chicken=12)

Struct(
	'pet_1' / animal(Int32ul),
	'pet_2' / animal(Int32ul),
	'pet_3' / animal(Computed(lambda this: 4 if this.pet_1 == 'cat' else 12)),
	'pet_4' / Computed(lambda this: 'dog' if this.pet_1 == 'cat' else 'chicken'),
)

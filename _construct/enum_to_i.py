def animal(subcon):
	return Enum(subcon, dog=4, cat=7, chicken=12)

Struct(
	'pet_1' / animal(Int32ul),
	'pet_2' / animal(Int32ul),
	'pet_1_i' / Computed(lambda this: int(this.pet_1)),
	'pet_1_mod' / Computed(lambda this: int(this.pet_1) + 0x8000),
	'one_lt_two' / Computed(lambda this: int(this.pet_1) < int(this.pet_2)),
)

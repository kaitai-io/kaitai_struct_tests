def animal(subcon):
	return Enum(subcon, 
		# -orig-id: MH_CANINE
		# A member of genus Canis.
		dog=4, 
		# -orig-id: MH_FELINE
		# Small, typically furry, carnivorous mammal.
		cat=7, 
		chicken=12,
	)

Struct(
	'pet_1' / animal(Int32ul),
	'pet_2' / animal(Int32ul),
)

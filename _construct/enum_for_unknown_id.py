# if parses an integer that isnt mapped, it returns an integer not string
def animal(subcon):
	return Enum(subcon, dog=4, cat=7, chicken=12)

Struct(
	'one' / animal(Int8ub),
)

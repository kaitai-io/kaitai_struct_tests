type_u1 = Struct(
	'count' / Int8ul,
	'values' / Array(this.count, Int8ul),
)

type_u2 = Struct(
	'count' / Int16ul,
	'values' / Array(this.count, Int16ul),
)

Struct(
	'first' / RepeatUntil(lambda obj_,list_,this: obj_.count == 0, type_u1),
	'second' / RepeatUntil(lambda obj_,list_,this: obj_.count == 0, type_u2),
	'third' / RepeatUntil(lambda obj_,list_,this: obj_ == 0, Int8ul),
)

# list_ is a list of all obj_ parsed so far (including current obj_). 
# I think this has no equivalent in KSY so it could be safely ignored.

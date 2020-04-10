local luaunit = require("luaunit")

require("enum_invalid")

TestEnumInvalid = {}

function TestEnumInvalid:test_enum_invalid()
    local r = EnumInvalid:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.pet_1, EnumInvalid.Animal.dog)
    luaunit.assertNil(r.pet_2)
end

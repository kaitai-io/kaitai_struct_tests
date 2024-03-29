-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("enum_to_i")

TestEnumToI = {}

function TestEnumToI:test_enum_to_i()
    local r = EnumToI:from_file("src/enum_0.bin")

    luaunit.assertEquals(r.pet_1, EnumToI.Animal.cat)
    luaunit.assertEquals(r.pet_2, EnumToI.Animal.chicken)
    luaunit.assertEquals(r.pet_1_i, 7)
    luaunit.assertEquals(r.pet_1_i_to_s, "7")
    luaunit.assertEquals(r.pet_1_mod, 32775)
    luaunit.assertEquals(r.one_lt_two, true)
    luaunit.assertEquals(r.pet_1_eq_int, true)
    luaunit.assertEquals(r.pet_2_eq_int, false)
end

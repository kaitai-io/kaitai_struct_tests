local luaunit = require("luaunit")

require("repeat_until_complex")

TestRepeatUntilComplex = {}

function TestRepeatUntilComplex:test_repeat_until_complex()
    local r = RepeatUntilComplex:from_file("src/repeat_until_complex.bin")

    luaunit.assertEquals(#r.first, 3)
    luaunit.assertEquals(r.first[1].count, 4)
    luaunit.assertEquals(r.first[1].values, {1, 2, 3, 4})
    luaunit.assertEquals(r.first[2].count, 2)
    luaunit.assertEquals(r.first[2].values, {1, 2})
    luaunit.assertEquals(r.first[3].count, 0)
    luaunit.assertEquals(r.first[3].values, {})

    luaunit.assertEquals(#r.second, 4)
    luaunit.assertEquals(r.second[1].count, 6)
    luaunit.assertEquals(r.second[1].values, {1, 2, 3, 4, 5, 6})
    luaunit.assertEquals(r.second[2].count, 3)
    luaunit.assertEquals(r.second[2].values, {1, 2, 3})
    luaunit.assertEquals(r.second[3].count, 4)
    luaunit.assertEquals(r.second[3].values, {1, 2, 3, 4})
    luaunit.assertEquals(r.second[4].count, 0)
    luaunit.assertEquals(r.second[4].values, {})

    luaunit.assertEquals(r.third, {102, 111, 111, 98, 97, 114, 0})
end

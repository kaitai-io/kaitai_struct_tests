local luaunit = require("luaunit")

require("debug_0")

TestDebug0 = {}

function TestDebug0:test_debug_0()
    local r = Debug0:from_file("src/fixed_struct.bin")

    -- --debug implies --no-auto-read
    r:_read()

    luaunit.assertEquals(r.one, 80)
    luaunit.assertEquals(#r.array_of_ints, 3)
    luaunit.assertEquals(r.array_of_ints[0 + 1], 65)
    luaunit.assertEquals(r.array_of_ints[1 + 1], 67)
    luaunit.assertEquals(r.array_of_ints[2 + 1], 75)
    luaunit.assertEquals(r._unnamed2, 45)

    -- FIXME: also test --read-pos once it is implemented
end

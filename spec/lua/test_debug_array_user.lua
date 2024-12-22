local luaunit = require("luaunit")

require("debug_array_user")

TestDebugArrayUser = {}

function TestDebugArrayUser:test_debug_array_user()
    local r = DebugArrayUser:from_file("src/fixed_struct.bin")

    -- --debug implies --no-auto-read
    r:_read()

    luaunit.assertEquals(r.one_cat.meow, 80)
    luaunit.assertEquals(#r.array_of_cats, 3)
    luaunit.assertEquals(r.array_of_cats[0 + 1].meow, 65)
    luaunit.assertEquals(r.array_of_cats[1 + 1].meow, 67)
    luaunit.assertEquals(r.array_of_cats[2 + 1].meow, 75)
end

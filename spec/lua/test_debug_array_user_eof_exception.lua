local luaunit = require("luaunit")

require("debug_array_user_eof_exception")

TestDebugArrayUserEofException = {}

function TestDebugArrayUserEofException:test_debug_array_user_eof_exception()
    local r = DebugArrayUserEofException:from_file("src/nav_parent_codes.bin")

    luaunit.assertErrorMsgMatches(".+: requested %d+ bytes, but only %d+ bytes available", function()
        -- --debug implies --no-auto-read
        r:_read()
    end)

    luaunit.assertEquals(r.one_cat.meow, 3)
    luaunit.assertEquals(r.one_cat.chirp, 73)
    luaunit.assertEquals(#r.array_of_cats, 3)
    luaunit.assertEquals(r.array_of_cats[0 + 1].meow, 49)
    luaunit.assertEquals(r.array_of_cats[0 + 1].chirp, 50)
    luaunit.assertEquals(r.array_of_cats[1 + 1].meow, 51)
    luaunit.assertEquals(r.array_of_cats[1 + 1].chirp, 66)
    luaunit.assertEquals(r.array_of_cats[2 + 1].meow, 98)
end

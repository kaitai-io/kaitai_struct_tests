local luaunit = require("luaunit")

require("debug_array_user_current_excluded")

TestDebugArrayUserCurrentExcluded = {}

function TestDebugArrayUserCurrentExcluded:test_debug_array_user_current_excluded()
    local r = DebugArrayUserCurrentExcluded:from_file("src/term_strz.bin")

    -- --debug implies --no-auto-read
    r:_read()

    luaunit.assertEquals(r.array_of_cats[0 + 1].meow, "\102\111\111")
    luaunit.assertEquals(r.array_of_cats[1 + 1].meow, "\124\098")
    luaunit.assertEquals(r.array_of_cats[2 + 1].meow, "\097")
end

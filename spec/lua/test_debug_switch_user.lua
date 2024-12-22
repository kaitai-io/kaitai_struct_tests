local luaunit = require("luaunit")

require("debug_switch_user")

TestDebugSwitchUser = {}

function TestDebugSwitchUser:test_debug_switch_user()
    local r = DebugSwitchUser:from_file("src/nav_parent_switch.bin")

    -- --debug implies --no-auto-read
    r:_read()

    luaunit.assertEquals(r.code, 1)
    luaunit.assertEquals(r.data.val, -190)
end

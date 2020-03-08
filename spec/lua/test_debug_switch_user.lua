-- runs in debug mode, so _read() needs to be called manually

local luaunit = require("luaunit")

require("debug_switch_user")

TestDebugSwitchUser = {}

function TestDebugSwitchUser:test_debug_switch_user()
    local r = DebugSwitchUser:from_file("src/nav_parent_switch.bin")
    r:_read()

    luaunit.assertEquals(r.code, 1)
    luaunit.assertEquals(r.data.val, -190)
end

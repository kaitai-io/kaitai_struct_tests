local luaunit = require("luaunit")

require("switch_cast")

TestSwitchCast = {}

function TestSwitchCast:test_switch_cast()
    local r = SwitchCast:from_file("src/switch_opcodes.bin")

    luaunit.assertEquals(r.first_obj.value, "foobar")
    luaunit.assertEquals(r.second_val, 0x42)
    -- unable to test "err_cast" here
end

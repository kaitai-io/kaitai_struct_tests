local luaunit = require("luaunit")

require("switch_repeat_expr")

TestSwitchRepeatExpr = {}

function TestSwitchRepeatExpr:test_switch_repeat_expr()
    local r = SwitchRepeatExpr:from_file("src/switch_tlv.bin")

    luaunit.assertEquals(r.code, 0x11)
    luaunit.assertEquals(r.size, 9)
    luaunit.assertEquals(r.body[1].first, "Stuff\000Me\000")
end

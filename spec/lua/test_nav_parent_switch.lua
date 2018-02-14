local luaunit = require("luaunit")

require("nav_parent_switch")

TestNavParentSwitch = {}

function TestNavParentSwitch:test_nav_parent_switch()
    local r = NavParentSwitch:from_file("src/nav_parent_switch.bin")

    luaunit.assertEquals(r.category, 1)
    luaunit.assertEquals(r.content.foo, 0x42)
    luaunit.assertEquals(r.content.subelement.bar, 0xff)
end

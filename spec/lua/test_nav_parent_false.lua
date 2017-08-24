local luaunit = require("luaunit")

require("nav_parent_false")

TestNavParentFalse = {}

function TestNavParentFalse:test_nav_parent_false()
    local r = NavParentFalse:from_file("src/nav_parent_codes.bin")

    luaunit.assertEquals(r.child_size, 3)
    luaunit.assertEquals(r.element_a.foo.code, 73)
    luaunit.assertEquals(r.element_a.foo.more, "123")
    luaunit.assertEquals(r.element_a.bar.foo.code, 66)
    luaunit.assertEquals(r.element_b.foo.code, 98)
end

local luaunit = require("luaunit")

require("nav_parent3")

TestNavParent3 = {}

function TestNavParent3:test_nav_parent3()
    local r = NavParent3:from_file("src/nav_parent2.bin")

    luaunit.assertEquals(r.ofs_tags, 8)
    luaunit.assertEquals(r.num_tags, 2)

    luaunit.assertEquals(r.tags[1].name, "RAHC")
    luaunit.assertEquals(r.tags[1].ofs, 0x20)
    luaunit.assertEquals(r.tags[1].num_items, 3)
    luaunit.assertEquals(r.tags[1].tag_content.content, "foo")

    luaunit.assertEquals(r.tags[2].name, "RAHC")
    luaunit.assertEquals(r.tags[2].ofs, 0x23)
    luaunit.assertEquals(r.tags[2].num_items, 6)
    luaunit.assertEquals(r.tags[2].tag_content.content, "barbaz")
end

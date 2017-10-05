local luaunit = require("luaunit")

require("imports_rel_1")

TestImportsRel1 = {}

function TestImportsRel1:test_imports_rel1()
    local r = ImportsRel1:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one, 0x50)
    luaunit.assertEquals(r.two.one, 0x41)
    luaunit.assertEquals(r.two.two.one, 0x43)
end

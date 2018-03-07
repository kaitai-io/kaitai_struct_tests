local luaunit = require("luaunit")

require("imports_abs_abs")

TestImportsAbsAbs = {}

function TestImportsAbsAbs:test_imports_abs_abs()
    local r = ImportsAbsAbs:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one, 0x50)
    luaunit.assertEquals(r.two.one, 0x41)
    luaunit.assertEquals(r.two.two.one, 0x43)
end

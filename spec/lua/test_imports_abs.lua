local luaunit = require("luaunit")

require("imports_abs")

TestImportsAbs = {}

function TestImportsAbs:test_import_abs()
    local r = ImportsAbs:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.len.value, 80)
    luaunit.assertEquals(r.body:len(), 80)
end

local luaunit = require("luaunit")

require("cast_to_imported")

TestCastToImported = {}

function TestCastToImported:test_cast_to_imported()
    local r = CastToImported:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one.one, 0x50)
    luaunit.assertEquals(r.one_casted.one, 0x50)
end

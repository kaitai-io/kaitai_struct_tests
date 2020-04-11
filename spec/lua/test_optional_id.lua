local luaunit = require("luaunit")

require("optional_id")

TestOptionalId = {}

function TestOptionalId:test_optional_id()
    local r = OptionalId:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r._unnamed0, 80)
    luaunit.assertEquals(r._unnamed1, 65)
    luaunit.assertEquals(r._unnamed2, "\067\075\045\049\255")
end

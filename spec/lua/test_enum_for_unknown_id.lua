local luaunit = require("luaunit")

require("enum_for_unknown_id")

TestEnumForUnknownId = {}

function TestEnumForUnknownId:test_enum_for_unknown_id()
    local r = EnumForUnknownId:from_file("src/fixed_struct.bin")

    luaunit.assertNil(r.one)
end

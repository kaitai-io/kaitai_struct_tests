local luaunit = require("luaunit")

require("enum_for_unknown_id")

TestEnumForUnknownId = {}

function TestEnumForUnknownId:test_enum_for_unknown_id()
    luaunit.assertError(EnumForUnknownId.from_file, "src/fixed_struct.bin")
end

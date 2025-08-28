local luaunit = require("luaunit")

require("debug_enum_name")

TestDebugEnumName = {}

function TestDebugEnumName:test_debug_enum_name()
    local r = DebugEnumName:from_file("src/fixed_struct.bin")

    -- --debug implies --no-auto-read
    r:_read()

    luaunit.assertEquals(r.one, DebugEnumName.TestEnum1.enum_value_80)
    luaunit.assertEquals(r.array_of_ints[0 + 1], DebugEnumName.TestEnum2.enum_value_65)
    luaunit.assertEquals(r.test_type.field1, DebugEnumName.TestSubtype.InnerEnum1.enum_value_67)
    luaunit.assertEquals(r.test_type.instance_field, DebugEnumName.TestSubtype.InnerEnum2.enum_value_11)
end

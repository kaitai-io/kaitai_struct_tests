local luaunit = require("luaunit")

require("debug_enum_name")

TestDebugEnumName = {}

function TestDebugEnumName:test_debug_enum_name()
    local r = DebugEnumName:from_file("src/fixed_struct.bin")

    -- This test is meaningful only for languages that have --debug and do
    -- not save enum type info
end

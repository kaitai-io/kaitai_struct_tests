-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("nested_type_param")

TestNestedTypeParam = {}

function TestNestedTypeParam:test_nested_type_param()
    local r = NestedTypeParam:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.main_seq.my_len, 5)
    luaunit.assertEquals(r.main_seq.body, "foo|b")
end

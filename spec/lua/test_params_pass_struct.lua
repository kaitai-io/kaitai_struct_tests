-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("params_pass_struct")

TestParamsPassStruct = {}

function TestParamsPassStruct:test_params_pass_struct()
    local r = ParamsPassStruct:from_file("src/enum_negative.bin")

    luaunit.assertEquals(r.first.foo, 255)
    luaunit.assertEquals(r.one.bar.qux, 1)
    luaunit.assertEquals(r.one.foo.foo, 255)
    luaunit.assertEquals(r.one.bar.foo.foo, 255)
end
-- Autogenerated from KST: please remove this line if doing any edits by hand!

local luaunit = require("luaunit")

require("index_to_param_until")

TestIndexToParamUntil = {}

function TestIndexToParamUntil:test_index_to_param_until()
    local r = IndexToParamUntil:from_file("src/index_sizes.bin")

    luaunit.assertEquals(r.qty, 3)
    luaunit.assertEquals(r.sizes[0 + 1], 1)
    luaunit.assertEquals(r.sizes[1 + 1], 8)
    luaunit.assertEquals(r.sizes[2 + 1], 4)
    luaunit.assertEquals(r.blocks[0 + 1].buf, "A")
    luaunit.assertEquals(r.blocks[1 + 1].buf, "BBBBBBBB")
    luaunit.assertEquals(r.blocks[2 + 1].buf, "CCCC")
end

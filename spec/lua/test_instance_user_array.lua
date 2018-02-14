local luaunit = require("luaunit")

require("instance_user_array")

TestInstanceUserArray = {}

function TestInstanceUserArray:test_instance_user_array()
    local r = InstanceUserArray:from_file("src/instance_std_array.bin")

    luaunit.assertEquals(r.ofs, 0x10)
    luaunit.assertEquals(r.qty_entries, 3)
    luaunit.assertEquals(r.entry_size, 4)

    luaunit.assertEquals(r.user_entries[1].word1, 0x1111)
    luaunit.assertEquals(r.user_entries[1].word2, 0x1111)
    luaunit.assertEquals(r.user_entries[2].word1, 0x2222)
    luaunit.assertEquals(r.user_entries[2].word2, 0x2222)
    luaunit.assertEquals(r.user_entries[3].word1, 0x3333)
    luaunit.assertEquals(r.user_entries[3].word2, 0x3333)
end

local luaunit = require("luaunit")

require("instance_std_array")

TestInstanceStdArray = {}

function TestInstanceStdArray:test_instance_std_array()
    local r = InstanceStdArray:from_file("src/instance_std_array.bin")

    luaunit.assertEquals(r.ofs, 0x10)
    luaunit.assertEquals(r.qty_entries, 3)
    luaunit.assertEquals(r.entry_size, 4)

    luaunit.assertEquals(r.entries, {
        "\x11\x11\x11\x11",
        "\x22\x22\x22\x22",
        "\x33\x33\x33\x33",
    })
end

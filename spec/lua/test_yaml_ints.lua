local luaunit = require("luaunit")

require("yaml_ints")

TestYamlInts = {}

function TestYamlInts:test_yaml_ints()
    local r = YamlInts:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.test_u4_dec, 0xffffffff)
    luaunit.assertEquals(r.test_u4_hex, 0xffffffff)
    luaunit.assertEquals(r.test_u8_dec, 0xffffffffffffffff)
    luaunit.assertEquals(r.test_u8_hex, 0xffffffffffffffff)
end

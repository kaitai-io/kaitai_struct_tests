local luaunit = require("luaunit")

require("process_xor4_value")

TestProcessXor4Value = {}

function TestProcessXor4Value:test_process_xor4_value()
    local r = ProcessXor4Value:from_file("src/process_xor_4.bin")

    luaunit.assertEquals(r.key, "\xec\xbb\xa3\x14")
    luaunit.assertEquals(r.buf, "foo bar")
end

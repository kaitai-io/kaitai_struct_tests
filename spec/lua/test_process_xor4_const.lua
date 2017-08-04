local luaunit = require("luaunit")

require("process_xor4_const")

TestProcessXor4Const = {}

function TestProcessXor4Const:test_process_xor4_const()
    local r = ProcessXor4Const:from_file("src/process_xor_4.bin")

    luaunit.assertEquals(r.key, "\xec\xbb\xa3\x14")
    luaunit.assertEquals(r.buf, "foo bar")
end

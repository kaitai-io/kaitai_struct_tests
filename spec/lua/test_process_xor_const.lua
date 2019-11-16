local luaunit = require("luaunit")

require("process_xor_const")

TestProcessXorConst = {}

function TestProcessXorConst:test_process_xor_const()
    local r = ProcessXorConst:from_file("src/process_xor_1.bin")

    luaunit.assertEquals(r.key, 0xff)
    luaunit.assertEquals(r.buf, "foo bar")
end

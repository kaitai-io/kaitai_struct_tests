local luaunit = require("luaunit")

require("process_xor_value")

TestProcessXorValue = {}

function TestProcessXorValue:test_process_xor_value()
    local r = ProcessXorValue:from_file("src/process_xor_1.bin")

    luaunit.assertEquals(r.key, 0xff)
    luaunit.assertEquals(r.buf, "foo bar")
end

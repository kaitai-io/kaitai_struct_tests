local luaunit = require("luaunit")

require("process_custom")

TestProcessCustom = {}

function TestProcessCustom:test_process_custom()
    local r = ProcessCustom:from_file("src/process_rotate.bin")

    luaunit.assertEquals(r.buf1, "\x10\xb3\x94\x94\xf4")
    luaunit.assertEquals(r.buf2, "\x5f\xba\x7b\x93\x63\x23\x5f")
    luaunit.assertEquals(r.buf3, "\x29\x33\xb1\x38\xb1")
end

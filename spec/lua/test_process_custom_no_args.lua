local luaunit = require("luaunit")

require("process_custom_no_args")

TestProcessCustomNoArgs = {}

function TestProcessCustomNoArgs:test_process_custom_no_args()
    local r = ProcessCustomNoArgs:from_file("src/process_rotate.bin")

    luaunit.assertEquals(r.buf1, "\x5F\x09\xAC\x8D\x8D\xED\x5F")
end

local luaunit = require("luaunit")

require("process_coerce_bytes")

TestProcessCoerceBytes = {}

function TestProcessCoerceBytes:test_process_coerce_bytes()
    local r = ProcessCoerceBytes:from_file("src/process_coerce_bytes.bin")

    luaunit.assertEquals(r.records[1].flag, 0)
    luaunit.assertEquals(r.records[1].buf, "AAAA")
    luaunit.assertEquals(r.records[2].flag, 1)
    luaunit.assertEquals(r.records[2].buf, "BBBB")
end

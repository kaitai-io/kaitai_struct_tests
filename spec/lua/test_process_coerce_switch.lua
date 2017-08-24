local luaunit = require("luaunit")

require("process_coerce_switch")

TestProcessCoerceSwitch = {}

function TestProcessCoerceSwitch:test_process_coerce_switch()
    local r = ProcessCoerceSwitch:from_file("src/process_coerce_switch.bin")

    luaunit.assertEquals(r.buf_type, 0)
    luaunit.assertEquals(r.flag, 0)
    luaunit.assertEquals(r.buf.bar, "AAAA")
end

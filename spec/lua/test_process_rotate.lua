local luaunit = require("luaunit")

require("process_rotate")

TestProcessRotate = {}

function TestProcessRotate:test_process_rotate()
    local r = ProcessRotate:from_file("src/process_rotate.bin")

    luaunit.assertEquals(r.buf1, "Hello")
    luaunit.assertEquals(r.buf2, "World")
    luaunit.assertEquals(r.buf3, "There")
end

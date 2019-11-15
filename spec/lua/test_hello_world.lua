local luaunit = require("luaunit")

require("hello_world")

TestHelloWorld = {}

function TestHelloWorld:test_hello_world()
    local r = HelloWorld:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.one, 0x50)
end

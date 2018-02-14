local luaunit = require("luaunit")

require("position_to_end")

TestPositionToEnd = {}

function TestPositionToEnd:test_position_to_end()
    local r = PositionToEnd:from_file("src/position_to_end.bin")

    luaunit.assertEquals(r.index.foo, 0x42)
    luaunit.assertEquals(r.index.bar, 0x1234)
end

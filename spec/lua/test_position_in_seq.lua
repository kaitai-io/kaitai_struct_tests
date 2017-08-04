local luaunit = require("luaunit")

require("position_in_seq")

TestPositionInSeq = {}

function TestPositionInSeq:test_position_in_seq()
    local r = PositionInSeq:from_file("src/position_in_seq.bin")

    luaunit.assertEquals(r.numbers, {1, 2, 3})
end

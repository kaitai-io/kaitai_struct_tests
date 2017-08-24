local luaunit = require("luaunit")

require("float_to_i")

TestFloatToI = {}

function TestFloatToI:test_float_to_i()
    local r = FloatToI:from_file("src/floating_points.bin")

    luaunit.assertEquals(r.single_value, 0.5)
    luaunit.assertEquals(r.double_value, 0.25)

    luaunit.assertEquals(r.single_i, 0)
    luaunit.assertEquals(r.double_i, 0)
    luaunit.assertEquals(r.float1_i, 1)
    luaunit.assertEquals(r.float2_i, 1)
    luaunit.assertEquals(r.float3_i, 1)
    luaunit.assertEquals(r.float4_i, -2)
end

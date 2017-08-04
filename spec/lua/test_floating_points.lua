local luaunit = require("luaunit")

require("floating_points")

TestFloatingPoints = {}

function TestFloatingPoints:test_floating_points()
    local r = FloatingPoints:from_file("src/floating_points.bin")
    local prec = 0.000001

    luaunit.assertEquals(r.single_value, 0.5)
    luaunit.assertEquals(r.single_value_be, 0.5)
    luaunit.assertEquals(r.double_value, 0.25)
    luaunit.assertEquals(r.double_value_be, 0.25)

    luaunit.assertAlmostEquals(r.approximate_value, 1.2345, prec)

    luaunit.assertAlmostEquals(r.single_value_plus_int, 1.5, prec)
    luaunit.assertAlmostEquals(r.single_value_plus_float, 1.0, prec)
    luaunit.assertAlmostEquals(r.double_value_plus_float, 0.3, prec)
end

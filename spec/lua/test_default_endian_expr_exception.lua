local luaunit = require("luaunit")

require("default_endian_expr_exception")

TestDefaultEndianExprException = {}

function TestDefaultEndianExprException:test_default_endian_expr_exception()
    luaunit.assertError(DefaultEndianExprException.from_file, "src/endian_expr.bin")
end

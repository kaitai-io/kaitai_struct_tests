local luaunit = require("luaunit")

require("eof_exception_u4")

TestEofExceptionU4 = {}

function TestEofExceptionU4:test_eof_exception_u4()
    luaunit.assertError(EofExceptionU4.from_file, "src/term_strz.bin")
end

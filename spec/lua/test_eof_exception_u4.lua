local luaunit = require("luaunit")

require("eof_exception_u4")

TestEofExceptionU4 = {}

function TestEofExceptionU4:test_eof_exception_u4()
    luaunit.assertErrorMsgMatches(".+: requested %d+ bytes, but got only %d+ bytes", EofExceptionU4.from_file, EofExceptionU4, "src/term_strz.bin")
end

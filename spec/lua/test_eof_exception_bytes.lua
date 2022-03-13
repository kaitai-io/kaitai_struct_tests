local luaunit = require("luaunit")

require("eof_exception_bytes")

TestEofExceptionBytes = {}

function TestEofExceptionBytes:test_eof_exception_bytes()
    luaunit.assertErrorMsgMatches(".+: requested %d+ bytes, but got only %d+ bytes", EofExceptionBytes.from_file, EofExceptionBytes, "src/term_strz.bin")
end

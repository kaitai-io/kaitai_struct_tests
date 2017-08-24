local luaunit = require("luaunit")

require("eof_exception_bytes")

TestEofExceptionBytes = {}

function TestEofExceptionBytes:test_eof_exception_bytes()
    luaunit.assertError(EofExceptionBytes.from_file, "src/term_strz.bin")
end

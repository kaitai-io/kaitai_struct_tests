local luaunit = require("luaunit")

require("eos_exception_bytes")

TestEosExceptionBytes = {}

function TestEosExceptionBytes:test_eos_exception_bytes()
    luaunit.assertErrorMsgMatches(".+: requested %d+ bytes, but got only %d+ bytes", EosExceptionBytes.from_file, EosExceptionBytes, "src/term_strz.bin")
end

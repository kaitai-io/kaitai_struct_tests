local luaunit = require("luaunit")

require("eos_exception_bytes")

TestEosExceptionBytes = {}

function TestEosExceptionBytes:test_eos_exception_bytes()
    luaunit.assertError(EosExceptionBytes.from_file, "src/term_strz.bin")
end

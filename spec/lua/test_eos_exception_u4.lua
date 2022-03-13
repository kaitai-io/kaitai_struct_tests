local luaunit = require("luaunit")

require("eos_exception_u4")

TestEosExceptionU4 = {}

function TestEosExceptionU4:test_eos_exception_u4()
    luaunit.assertErrorMsgMatches(".+: requested %d+ bytes, but got only %d+ bytes", EosExceptionU4.from_file, EosExceptionU4, "src/term_strz.bin")
end

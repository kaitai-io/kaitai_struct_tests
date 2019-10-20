local luaunit = require("luaunit")

require("eos_exception_u4")

TestEosExceptionU4 = {}

function TestEosExceptionU4:test_eos_exception_u4()
    luaunit.assertError(EosExceptionU4.from_file, "src/term_strz.bin")
end

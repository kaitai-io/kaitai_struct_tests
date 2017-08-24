local luaunit = require("luaunit")

require("params_call_short")

TestParamsCallShort = {}

function TestParamsCallShort:test_params_call_short()
    local r = ParamsCallShort:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.buf1.body, "foo|b")
    luaunit.assertEquals(r.buf2.body, "ar|ba")
    luaunit.assertEquals(r.buf2.trailer, 0x7a)
end

local luaunit = require("luaunit")

require("params_def")
require("kaitaistruct")

TestParamsDef = {}

function TestParamsDef:test_params_def()
    local _io = KaitaiStream(io.open("src/term_strz.bin", "rb"))
    local r = ParamsDef(5, true, _io, nil, nil)

    luaunit.assertEquals(r.buf, "foo|b")
    luaunit.assertEquals(r.trailer, 0x61)

    _io:close()    
end

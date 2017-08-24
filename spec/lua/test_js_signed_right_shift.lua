local luaunit = require("luaunit")

require("js_signed_right_shift")

TestJsSignedRightShift = {}

function TestJsSignedRightShift:test_js_signed_right_shift()
    local r = JsSignedRightShift:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(r.should_be_40000000, 0x40000000)
    luaunit.assertEquals(r.should_be_a00000, 0xa00000)
end

local luaunit = require("luaunit")

require("str_encodings")

TestStrEncodings = {}

function TestStrEncodings:test_str_encodings()
    local r = StrEncodings:from_file("src/str_encodings.bin")

    luaunit.assertEquals(r.str1, "Some ASCII")
    luaunit.assertEquals(r.str2, "こんにちは")
    luaunit.assertEquals(r.str3, "こんにちは")
    luaunit.assertEquals(r.str4, "░▒▓")
end

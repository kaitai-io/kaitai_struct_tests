local luaunit = require("luaunit")

require("str_encodings_default")

TestStrEncodingsDefault = {}

function TestStrEncodingsDefault:test_str_encodings_default()
    local r = StrEncodingsDefault:from_file("src/str_encodings.bin")

    luaunit.assertEquals(r.str1, "Some ASCII")
    luaunit.assertEquals(r.rest.str2, u"こんにちは")
    luaunit.assertEquals(r.rest.str3, u"こんにちは")
    luaunit.assertEquals(r.rest.str4, u"░▒▓")
end

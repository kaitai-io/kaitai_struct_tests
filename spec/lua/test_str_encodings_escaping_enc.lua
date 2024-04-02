local luaunit = require("luaunit")

require("str_encodings_escaping_enc")

TestStrEncodingsEscapingEnc = {}

local function assertUnknownEncoding(expected_encoding, fn)
    luaunit.assertErrorMsgContains("Encoding " .. expected_encoding .. " not supported", fn)
end

function TestStrEncodingsEscapingEnc:test_str_encodings_escaping_enc()
    local r = StrEncodingsEscapingEnc:from_file("src/str_encodings.bin")

    assertUnknownEncoding(
        "ASCII\\\\x",
        function()
            local _ = r.str1.v
        end
    )
    assertUnknownEncoding(
        "UTF-8\\'x",
        function()
            local _ = r.str2.v
        end
    )
    assertUnknownEncoding(
        "SJIS\\\"x",
        function()
            local _ = r.str3.v
        end
    )
    assertUnknownEncoding(
        "IBM437\\nx",
        function()
            local _ = r.str4.v
        end
    )
end

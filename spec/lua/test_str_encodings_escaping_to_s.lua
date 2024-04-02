local luaunit = require("luaunit")

require("str_encodings_escaping_to_s")

TestStrEncodingsEscapingToS = {}

local function assertUnknownEncoding(expected_encoding, fn)
    luaunit.assertErrorMsgContains("Encoding " .. expected_encoding .. " not supported", fn)
end

function TestStrEncodingsEscapingToS:test_str_encodings_escaping_to_s()
    local r = StrEncodingsEscapingToS:from_file("src/str_encodings.bin")

    assertUnknownEncoding(
        "ASCII\\\\x",
        function()
            local _ = r.str1
        end
    )
    assertUnknownEncoding(
        "UTF-8\\'x",
        function()
            local _ = r.str2
        end
    )
    assertUnknownEncoding(
        "SJIS\\\"x",
        function()
            local _ = r.str3
        end
    )
    assertUnknownEncoding(
        "IBM437\\nx",
        function()
            local _ = r.str4
        end
    )
end

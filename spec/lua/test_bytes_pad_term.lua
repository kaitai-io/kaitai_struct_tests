local luaunit = require("luaunit")

require("bytes_pad_term")

TestBytesPadTerm = {}

function TestBytesPadTerm:test_bytes_pad_term()
    local r = BytesPadTerm:from_file("src/str_pad_term.bin")

    luaunit.assertEquals(r.str_pad, "str1")
    luaunit.assertEquals(r.str_term, "str2foo")
    luaunit.assertEquals(r.str_term_and_pad, "str+++3bar+++")
    luaunit.assertEquals(r.str_term_include, "str4baz@")
end

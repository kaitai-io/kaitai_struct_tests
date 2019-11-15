local luaunit = require("luaunit")

require("meta_xref")

TestMetaXref = {}

function TestMetaXref:test_meta_xref()
    local r = MetaXref:from_file("src/fixed_struct.bin")
end

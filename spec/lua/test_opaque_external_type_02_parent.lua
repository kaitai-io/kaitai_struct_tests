local luaunit = require("luaunit")

require("opaque_external_type_02_parent")

TestOpaqueExternalType02Parent = {}

function TestOpaqueExternalType02Parent:test_opaque_external_type_02_parent()
    local r = OpaqueExternalType02Parent:from_file("src/term_strz.bin")

    luaunit.assertEquals(r.parent.child.s1, "foo")
    luaunit.assertEquals(r.parent.child.s2, "bar")
    luaunit.assertEquals(r.parent.child.s3.s3, "|baz@")
end

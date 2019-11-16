local luaunit = require("luaunit")

require("switch_manual_int_size_eos")

TestSwitchManualIntSizeEos = {}

function TestSwitchManualIntSizeEos:test_switch_manual_int_size_eos()
    local r = SwitchManualIntSizeEos:from_file("src/switch_tlv.bin")

    luaunit.assertEquals(#r.chunks, 4)

    luaunit.assertEquals(r.chunks[1].code, 0x11)
    local meta = r.chunks[1].body.body
    luaunit.assertEquals(meta.title, "Stuff")
    luaunit.assertEquals(meta.author, "Me")

    luaunit.assertEquals(r.chunks[2].code, 0x22)
    luaunit.assertEquals(r.chunks[2].body.body.entries, {"AAAA", "BBBB", "CCCC"})

    luaunit.assertEquals(r.chunks[3].code, 0x33)
    luaunit.assertEquals(r.chunks[3].body.body, "\x10\x20\x30\x40\x50\x60\x70\x80")

    luaunit.assertEquals(r.chunks[4].code, 0xff)
    luaunit.assertEquals(r.chunks[4].body.body, "")
end

local luaunit = require("luaunit")

require("ts_packet_header")

TestTsPacketHeader = {}

function TestTsPacketHeader:test_ts_packet_header()
    local r = TsPacketHeader:from_file("src/ts_packet.bin")

    luaunit.assertEquals(r.sync_byte, 0x47)
    luaunit.assertEquals(r.transport_error_indicator, 0)
    luaunit.assertEquals(r.payload_unit_start_indicator, 0)
    luaunit.assertEquals(r.transport_priority, 1)
    luaunit.assertEquals(r.pid, 33)
    luaunit.assertEquals(r.transport_scrambling_control, 0)
    luaunit.assertEquals(r.adaptation_field_control, TsPacketHeader.AdaptationFieldControlEnum.payload_only)
end

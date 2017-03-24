package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TsPacketHeader;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestTsPacketHeader extends CommonSpec {
    @Test
    public void testTsPacketHeader() throws Exception {
        TsPacketHeader r = TsPacketHeader.fromFile(SRC_DIR + "ts_packet.bin");

        assertEquals(r.syncByte(), 0x47);
        assertEquals(r.transportErrorIndicator(), false);
        assertEquals(r.payloadUnitStartIndicator(), false);
        assertEquals(r.transportPriority(), true);
        assertEquals(r.pid(), 33);
        assertEquals(r.transportScramblingControl(), 0);
        assertEquals(r.adaptationFieldControl(), TsPacketHeader.AdaptationFieldControlEnum.PAYLOAD_ONLY);
    }
}

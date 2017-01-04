package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsSimple;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBitsSimple extends CommonSpec {
    @Test
    public void testBitsSimple() throws Exception {
        BitsSimple r = BitsSimple.fromFile(SRC_DIR + "fixed_struct.bin");

        // 50 41
        assertEquals(r.byte1(), 0x50);
        assertEquals(r.byte2(), 0x41);

        // 43 (1 + 3 + 4) = 0|100|0011
        assertEquals(r.bitsA(), false);
        assertEquals(r.bitsB(), 0b100);
        assertEquals(r.bitsC(), 0b0011);

        // 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
        assertEquals(r.largeBits1(), 0b0100101100);
        assertEquals(r.spacer(), 0b101);
        assertEquals(r.largeBits2(), 0b10100110001);

        // FF FF
        assertEquals(r.normalS2(), -1);

        // 50 41 43
        assertEquals(r.byte8910(), 0x504143);
    }
}

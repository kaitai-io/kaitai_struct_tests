package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsByteAligned;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBitsByteAligned extends CommonSpec {
    @Test
    public void testBitsByteAligned() throws Exception {
        BitsByteAligned r = BitsByteAligned.fromFile(SRC_DIR + "fixed_struct.bin");

        // 50 (6 + 2) = 010100|00
        assertEquals(r.one(), 0b010100);
        // 41
        assertEquals(r.byte1(), 0x41);
        // 43 (3 + 1 + 4) = 010|0|0011
        assertEquals(r.two(), 0b010);
        assertEquals(r.three(), false);
        // 4B
        assertEquals(r.byte2(), 0x4b);
        // 2D 31 (14 + 2) = 00101101 001100|01
        assertEquals(r.four(), 0b00101101_001100);
        // FF
        assertEquals(r.byte3(), new byte[] { (byte) 0xff });
        // FF
        assertEquals(r.fullByte(), 0xff);
        // 50
        assertEquals(r.byte4(), 0x50);
    }
}

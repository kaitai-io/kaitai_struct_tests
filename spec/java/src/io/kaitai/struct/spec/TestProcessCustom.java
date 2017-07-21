package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessCustom;
import org.testng.annotations.Test;

import static org.testng.internal.junit.ArrayAsserts.assertArrayEquals;

public class TestProcessCustom extends CommonSpec {
    @Test
    public void testProcessCustom() throws Exception {
        ProcessCustom r = ProcessCustom.fromFile(SRC_DIR + "process_rotate.bin");

        assertArrayEquals(new byte[] {
                0x10, (byte) 0xb3, (byte) 0x94, (byte) 0x94, (byte) 0xf4
        }, r.buf1());
        assertArrayEquals(new byte[] {
                0x5f, (byte) 0xba, 0x7b, (byte) 0x93, 0x63, 0x23, 0x5f
        }, r.buf2());
        assertArrayEquals(new byte[] {
                0x29, 0x33, (byte) 0xb1, 0x38, (byte) 0xb1
        }, r.buf3());
    }
}

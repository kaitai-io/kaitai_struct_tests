package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TermBytes;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestTermBytes extends CommonSpec {
    @Test
    public void testTermBytes() throws Exception {
        TermBytes r = TermBytes.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.s1(), new byte[] { 0x66, 0x6f, 0x6f });
        assertEquals(r.s2(), new byte[] { 0x62, 0x61, 0x72 });
        assertEquals(r.s3(), new byte[] { 0x7c, 0x62, 0x61, 0x7a, 0x40 });
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.TermBytes;
import org.testng.annotations.Test;

public class TestTermBytes extends CommonSpec {
    @Test
    public void testTermBytes() throws Exception {
        TermBytes r = new TermBytes();

        r.setS1(new byte[] { 0x66, 0x6f, 0x6f });
        r.setS2(new byte[] { 0x62, 0x61, 0x72 });
        r.setS3(new byte[] { 0x7c, 0x62, 0x61, 0x7a, 0x40 });

        assertEqualToFile(r, "term_strz.bin");
    }
}

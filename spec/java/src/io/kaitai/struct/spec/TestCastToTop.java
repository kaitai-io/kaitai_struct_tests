package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.CastToTop;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestCastToTop extends CommonSpec {
    @Test
    public void testCastToTop() throws Exception {
        CastToTop r = CastToTop.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.code(), 0x50);
        assertEquals(r.header().code(), 0x41);
        assertEquals(r.headerCasted().code(), 0x41);
    }
}

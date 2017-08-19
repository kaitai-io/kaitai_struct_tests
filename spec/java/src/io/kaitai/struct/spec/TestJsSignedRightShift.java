package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.JsSignedRightShift;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestJsSignedRightShift extends CommonSpec {
    @Test
    public void testJsSignedRightShift() throws Exception {
        JsSignedRightShift r = JsSignedRightShift.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.shouldBe40000000().longValue(), 0x40000000L);
        assertEquals(r.shouldBeA00000().longValue(), 0xa00000L);
    }
}

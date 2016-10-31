package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIoPos;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExprIoPos extends CommonSpec {
    @Test
    public void testExprIoPos() throws Exception {
        ExprIoPos r = ExprIoPos.fromFile(SRC_DIR + "expr_io_pos.bin");

        assertEquals(r.substream1().myStr(), "CURIOSITY");
        assertEquals(r.substream1().body(), new byte[] { 0x11, 0x22, 0x33, 0x44 });
        assertEquals(r.substream1().number(), 0x42);

        assertEquals(r.substream2().myStr(), "KILLED");
        assertEquals(r.substream2().body(), new byte[] { 0x61, 0x20, 0x63, 0x61, 0x74 });
        assertEquals(r.substream2().number(), 0x67);
    }
}

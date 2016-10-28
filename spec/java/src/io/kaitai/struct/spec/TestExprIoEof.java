package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIoEof;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertNull;

public class TestExprIoEof extends CommonSpec {
    @Test
    public void testExprIoEof() throws Exception {
        ExprIoEof r = ExprIoEof.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.substream1().one(), 1262698832L);
        assertNull(r.substream1().two());

        assertEquals(r.substream2().one(), 4294914349L);
        assertEquals(r.substream2().two().longValue(), 1262698832L);
    }
}

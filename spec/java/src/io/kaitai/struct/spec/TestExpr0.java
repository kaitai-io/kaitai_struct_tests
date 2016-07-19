package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Expr0;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExpr0 extends CommonSpec {
    @Test
    public void testExpr0() throws Exception {
        Expr0 r = Expr0.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.mustBeF7().intValue(), 0xf7);
        assertEquals(r.mustBeAbc123(), "abc123");
    }
}

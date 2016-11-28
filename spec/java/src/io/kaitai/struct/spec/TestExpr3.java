package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Expr3;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExpr3 extends CommonSpec {
    @Test
    public void testExpr3() throws Exception {
        Expr3 r = Expr3.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 80);
        assertEquals(r.two(), "ACK");

        assertEquals(r.three(), "@ACK");
        assertEquals(r.four(), "_ACK_");
        assertEquals(r.isStrEq().booleanValue(), true);
        assertEquals(r.isStrNe().booleanValue(), false);
        assertEquals(r.isStrLt().booleanValue(), true);
        assertEquals(r.isStrGt().booleanValue(), false);
        assertEquals(r.isStrLe().booleanValue(), true);
        assertEquals(r.isStrGe().booleanValue(), false);
        assertEquals(r.isStrLt2().booleanValue(), true);
        assertEquals(r.testNot().booleanValue(), true);
    }
}

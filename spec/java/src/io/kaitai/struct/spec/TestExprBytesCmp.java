package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprBytesCmp;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExprBytesCmp extends CommonSpec {
    @Test
    public void testExprBytesCmp() throws Exception {
        ExprBytesCmp r = ExprBytesCmp.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), "P".getBytes());
        assertEquals(r.two(), "ACK".getBytes());

        assertEquals(r.isEq().booleanValue(), true);
        assertEquals(r.isNe().booleanValue(), false);
        assertEquals(r.isLt().booleanValue(), true);
        assertEquals(r.isGt().booleanValue(), false);
        assertEquals(r.isLe().booleanValue(), true);
        assertEquals(r.isGe().booleanValue(), false);
        assertEquals(r.isLt2().booleanValue(), false);
        assertEquals(r.isGt2().booleanValue(), true);
    }
}

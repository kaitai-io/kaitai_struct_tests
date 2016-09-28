package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprMod;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExprMod extends CommonSpec {
    @Test
    public void testExprMod() throws Exception {
        ExprMod r = ExprMod.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.intU(), 1262698832);
        assertEquals(r.intS(), -52947);

        assertEquals(r.modPosConst().intValue(), 9);
        assertEquals(r.modNegConst().intValue(), 4);
        assertEquals(r.modPosSeq().intValue(), 5);
        assertEquals(r.modNegSeq().intValue(), 2);
    }
}

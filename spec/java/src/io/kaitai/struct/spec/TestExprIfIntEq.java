// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIfIntEq;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestExprIfIntEq extends CommonSpec {
    @Test
    public void testExprIfIntEq() throws Exception {
        ExprIfIntEq r = ExprIfIntEq.fromFile(SRC_DIR + "process_coerce_switch.bin");

        assertIntEquals(r.seqEqLit(), true);
        assertIntEquals(r.seqEqCalc(), true);
        assertIntEquals(r.seqEqCalcIf(), true);
        assertIntEquals(r.seqEqSeqIf(), true);
        assertIntEquals(r.calcEqLit(), true);
        assertIntEquals(r.calcEqCalcIf(), true);
        assertIntEquals(r.calcEqSeqIf(), true);
        assertIntEquals(r.calcIfEqLit(), true);
        assertIntEquals(r.calcIfEqSeqIf(), true);
        assertIntEquals(r.seqIfEqLit(), true);
    }
}
// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIfIntOps;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestExprIfIntOps extends CommonSpec {

    @Test
    public void testExprIfIntOps() throws Exception {
        ExprIfIntOps r = ExprIfIntOps.fromFile(SRC_DIR + "process_coerce_switch.bin");

        assertIntEquals(r.isEqPrim(), true);
        assertIntEquals(r.isEqBoxed(), true);
    }
}

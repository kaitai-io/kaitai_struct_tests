// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprBytesNonLiteral;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestExprBytesNonLiteral extends CommonSpec {

    @Test
    public void testExprBytesNonLiteral() throws Exception {
        ExprBytesNonLiteral r = ExprBytesNonLiteral.fromFile(SRC_DIR + "enum_negative.bin");

        assertIntEquals(r.calcBytes().length, 2);
        assertIntEquals(r.calcBytes()[0], 255);
        assertIntEquals(r.calcBytes()[1], 1);
    }
}

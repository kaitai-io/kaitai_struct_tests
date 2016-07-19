package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestExpr1 extends CommonSpec {
    @Test
    public void testExpr1() throws Exception {
        Expr1 r = Expr1.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.lenOf1(), 10);
        assertEquals(r.lenOf1Mod().intValue(), 8);
        assertEquals(r.str1(), "Some ASC");
        assertEquals(r.str1Len().intValue(), 8);
    }

}

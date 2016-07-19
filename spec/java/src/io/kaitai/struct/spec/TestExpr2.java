package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestExpr2 extends CommonSpec {
    @Test
    public void testExpr2() throws Exception {
        Expr2 r = Expr2.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.str1().lenOrig(), 10);
        assertEquals(r.str1().lenMod().intValue(), 7);
        assertEquals(r.str1().str(), "Some AS");

        assertEquals(r.str1Len().intValue(), 7);
        assertEquals(r.str1LenMod().intValue(), 7);
        assertEquals(r.str1Byte1().intValue(), 0x49);
        assertEquals(r.str1Avg().intValue(), 0x49);
        assertEquals(r.str1Char5(), "e");

        assertEquals(r.str1Tuple5().byte0(), 0x65);
        assertEquals(r.str1Tuple5().byte1(), 0x20);
        assertEquals(r.str1Tuple5().byte2(), 0x41);
        assertEquals(r.str1Tuple5().avg().intValue(), 0x30);

        assertEquals(r.str2Tuple5().byte0(), 0x65);
        assertEquals(r.str2Tuple5().byte1(), 0x20);
        assertEquals(r.str2Tuple5().byte2(), 0x41);
        assertEquals(r.str2Tuple5().avg().intValue(), 0x30);
    }

}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprArray;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExprArray extends CommonSpec {
    @Test
    public void testExprArray() throws Exception {
        ExprArray r = ExprArray.fromFile(SRC_DIR + "expr_array.bin");

        assertEquals(r.aintSize().intValue(), 4);
        assertEquals(r.aintFirst().intValue(), 7657765);
        assertEquals(r.aintLast().intValue(), 16272640);
        assertEquals(r.aintMin().intValue(), 49185);
        assertEquals(r.aintMax().intValue(), 1123362332);

        assertEquals(r.afloatSize().intValue(), 3);
        assertEquals(r.afloatFirst().doubleValue(), -2.6839530254859364e-121);
        assertEquals(r.afloatLast().doubleValue(), -1.1103359815095273e-175);
        assertEquals(r.afloatMin().doubleValue(), -8.754689149998834e+288);
        assertEquals(r.afloatMax().doubleValue(), -1.1103359815095273e-175);

        assertEquals(r.astrSize().intValue(), 3);
        assertEquals(r.astrFirst(), "foo");
        assertEquals(r.astrLast(), "baz");
        assertEquals(r.astrMin(), "bar");
        assertEquals(r.astrMax(), "foo");
    }
}

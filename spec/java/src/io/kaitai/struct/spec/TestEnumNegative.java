package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumNegative;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestEnumNegative extends CommonSpec {
    @Test
    public void testEnumNegative() throws Exception {
        EnumNegative r = EnumNegative.fromFile(SRC_DIR + "enum_negative.bin");

        assertEquals(r.f1(), EnumNegative.Constants.NEGATIVE_ONE);
        assertEquals(r.f2(), EnumNegative.Constants.POSITIVE_ONE);
    }
}

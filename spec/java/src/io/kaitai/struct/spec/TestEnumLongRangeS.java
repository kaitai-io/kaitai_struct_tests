// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumLongRangeS;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestEnumLongRangeS extends CommonSpec {

    @Test
    public void testEnumLongRangeS() throws Exception {
        EnumLongRangeS r = EnumLongRangeS.fromFile(SRC_DIR + "enum_long_range_s.bin");

        assertEquals(r.f1(), EnumLongRangeS.Constants.LONG_MIN);
        assertEquals(r.f2(), EnumLongRangeS.Constants.INT_BELOW_MIN);
        assertEquals(r.f3(), EnumLongRangeS.Constants.INT_MIN);
        assertEquals(r.f4(), EnumLongRangeS.Constants.ZERO);
        assertEquals(r.f5(), EnumLongRangeS.Constants.INT_MAX);
        assertEquals(r.f6(), EnumLongRangeS.Constants.INT_OVER_MAX);
        assertEquals(r.f7(), EnumLongRangeS.Constants.LONG_MAX);
    }
}
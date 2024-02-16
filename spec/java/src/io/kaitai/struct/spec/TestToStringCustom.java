package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ToStringCustom;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestToStringCustom extends CommonSpec {
    @Test
    public void testToStringCustom() throws Exception {
        ToStringCustom r = ToStringCustom.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.toString(), "s1 = foo, s2 = bar");
    }
}

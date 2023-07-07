package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ToCustomString;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestToCustomString extends CommonSpec {
    @Test
    public void testToCustomString() throws Exception {
        ToCustomString r = ToCustomString.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.toString(), "s1 = foo, s2 = bar");
    }
}

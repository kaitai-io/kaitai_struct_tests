package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NonStandard;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNonStandard extends CommonSpec {
    @Test
    public void testNonStandard() throws Exception {
        NonStandard r = NonStandard.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.foo(), 0x50);
    }
}

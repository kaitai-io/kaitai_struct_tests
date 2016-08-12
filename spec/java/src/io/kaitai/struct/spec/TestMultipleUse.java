package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.MultipleUse;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestMultipleUse extends CommonSpec {
    @Test
    public void testMultipleUse() throws Exception {
        MultipleUse r = MultipleUse.fromFile(SRC_DIR + "position_abs.bin");

        assertEquals(r.t1().firstUse().value(), 0x20);
        assertEquals(r.t2().secondUse().value(), 0x20);
    }
}

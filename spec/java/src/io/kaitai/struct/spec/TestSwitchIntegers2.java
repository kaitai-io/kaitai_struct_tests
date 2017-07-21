package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchIntegers2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchIntegers2 extends CommonSpec {
    @Test
    public void testSwitchIntegers2() throws Exception {
        SwitchIntegers2 r = SwitchIntegers2.fromFile(SRC_DIR + "switch_integers.bin");

        assertEquals(r.code(), 1);
        assertEquals(r.len().longValue(), 7);
        assertEquals(r.ham(), new byte[] { 2, 64, 64, 4, 55, 19, 0 });
        assertEquals(r.padding().intValue(), 0);
        assertEquals(r.lenModStr(), "13");
    }
}

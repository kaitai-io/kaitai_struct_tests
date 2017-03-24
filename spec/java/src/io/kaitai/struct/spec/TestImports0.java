package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Imports0;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestImports0 extends CommonSpec {
    @Test
    public void testImports0() throws Exception {
        Imports0 r = Imports0.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.two(), 0x50);
        assertEquals(r.hw().one(), 0x41);
        assertEquals(r.hwOne().intValue(), 0x41);
    }
}

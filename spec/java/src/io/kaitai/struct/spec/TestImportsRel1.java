package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsRel1;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestImportsRel1 extends CommonSpec {
    @Test
    public void testImportsRel1() throws Exception {
        ImportsRel1 r = ImportsRel1.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
        assertEquals(r.two().one(), 0x41);
        assertEquals(r.two().two().one(), 0x43);
    }
}

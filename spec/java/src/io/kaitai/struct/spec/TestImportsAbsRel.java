package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsAbsRel;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestImportsAbsRel extends CommonSpec {
    @Test
    public void testImportsAbsRel() throws Exception {
        ImportsAbsRel r = ImportsAbsRel.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
        assertEquals(r.two().one(), 0x41);
        assertEquals(r.two().two().one(), 0x43);
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsAbsAbs;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestImportsAbsAbs extends CommonSpec {
    @Test
    public void testImportsAbsAbs() throws Exception {
        ImportsAbsAbs r = ImportsAbsAbs.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
        assertEquals(r.two().one(), 0x41);
        assertEquals(r.two().two().one(), 0x43);
    }
}

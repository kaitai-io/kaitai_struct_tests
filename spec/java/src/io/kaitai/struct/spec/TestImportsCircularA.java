package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsCircularA;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertNull;

public class TestImportsCircularA extends CommonSpec {
    @Test
    public void testImportsCircularA() throws Exception {
        ImportsCircularA r = ImportsCircularA.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.code(), 0x50);
        assertEquals(r.two().initial(), 0x41);
        assertEquals(r.two().backRef().code(), 0x43);
        assertEquals(r.two().backRef().two().initial(), 0x4b);
        assertNull(r.two().backRef().two().backRef());
    }
}

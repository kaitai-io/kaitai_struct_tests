package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NestedSameName2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNestedSameName2 extends CommonSpec {
    @Test
    public void testNestedSameName2() throws Exception {
        NestedSameName2 r = NestedSameName2.fromFile(SRC_DIR + "nested_same_name2.bin");

        assertEquals(r.version(), 0x42);
        assertEquals(r.mainData().mainSize(), 2);
        assertEquals(r.mainData().foo().data1(), new byte[] { 0x11, 0x11, 0x11, 0x11 });
        assertEquals(r.dummy().dummySize(), 3);
        assertEquals(r.dummy().foo().data2(), new byte[] { 0x22, 0x22, 0x22, 0x22, 0x22, 0x22 });
    }
}

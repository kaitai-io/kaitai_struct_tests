package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NestedTypes2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNestedTypes2 extends CommonSpec {
    @Test
    public void testNestedTypes2() throws Exception {
        NestedTypes2 r = NestedTypes2.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one().typedAtRoot().valueB(), 80);

        assertEquals(r.one().typedHere1().valueC(), 65);

        assertEquals(r.one().typedHere1().typedHere().valueD(), 67);
        assertEquals(r.one().typedHere1().typedParent().valueCc(), 75);
        assertEquals(r.one().typedHere1().typedRoot().valueB(), 45);

        assertEquals(r.one().typedHere2().valueCc(), 49);

        assertEquals(r.two().valueB(), -1);
    }
}

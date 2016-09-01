package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NestedTypes;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNestedTypes extends CommonSpec {
    @Test
    public void testNestedTypes() throws Exception {
        NestedTypes r = NestedTypes.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one().typedAtRoot().valueB(), 80);
        assertEquals(r.one().typedHere().valueC(), 65);
        assertEquals(r.two().valueB(), 67);
    }
}

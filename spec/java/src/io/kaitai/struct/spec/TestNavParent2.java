package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParent2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNavParent2 extends CommonSpec {
    @Test
    public void testNavParent2() throws Exception {
        NavParent2 r = NavParent2.fromFile(SRC_DIR + "nav_parent2.bin");

        assertEquals(r.ofsTags(), 8);
        assertEquals(r.numTags(), 2);

        assertEquals(r.tags().get(0).name(), "RAHC");
        assertEquals(r.tags().get(0).ofs(), 0x20);
        assertEquals(r.tags().get(0).numItems(), 3);
        assertEquals(r.tags().get(0).tagContent().content(), "foo");

        assertEquals(r.tags().get(1).name(), "RAHC");
        assertEquals(r.tags().get(1).ofs(), 0x23);
        assertEquals(r.tags().get(1).numItems(), 6);
        assertEquals(r.tags().get(1).tagContent().content(), "barbaz");
    }
}

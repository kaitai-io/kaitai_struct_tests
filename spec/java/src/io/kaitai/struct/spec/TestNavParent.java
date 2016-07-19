package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParent;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNavParent extends CommonSpec {
    @Test
    public void testNavParent() throws Exception {
        NavParent r = NavParent.fromFile(SRC_DIR + "nav.bin");

        assertEquals(r.header().qtyEntries(), 2);
        assertEquals(r.header().filenameLen(), 8);

        assertEquals(r.index().entries().size(), 2);
        assertEquals(r.index().entries().get(0).filename(), "FIRST___");
        assertEquals(r.index().entries().get(1).filename(), "SECOND__");
    }
}

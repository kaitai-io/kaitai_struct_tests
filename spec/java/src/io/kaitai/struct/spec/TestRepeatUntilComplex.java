package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatUntilComplex;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatUntilComplex extends CommonSpec {
    @Test
    public void testRepeatUntilComplex() throws Exception {
        RepeatUntilComplex r = RepeatUntilComplex.fromFile(SRC_DIR + "repeat_until_complex.bin");

        assertEquals(r.first().size(), 3);
        assertEquals(r.first().get(0).count(), 4);
        assertEquals(r.first().get(0).values().toArray(), new Integer[] { 1, 2, 3, 4 });
        assertEquals(r.first().get(1).count(), 2);
        assertEquals(r.first().get(1).values().toArray(), new Integer[] { 1, 2 });
        assertEquals(r.first().get(2).count(), 0);
        assertEquals(r.first().get(2).values().toArray(), new Integer[] {  });
        assertEquals(r.second().size(), 4);
        assertEquals(r.second().get(0).count(), 6);
        assertEquals(r.second().get(0).values().toArray(), new Integer[] { 1, 2, 3, 4, 5, 6 });
        assertEquals(r.second().get(1).count(), 3);
        assertEquals(r.second().get(1).values().toArray(), new Integer[] { 1, 2, 3 });
        assertEquals(r.second().get(2).count(), 4);
        assertEquals(r.second().get(2).values().toArray(), new Integer[] { 1, 2, 3, 4 });
        assertEquals(r.second().get(3).count(), 0);
        assertEquals(r.second().get(3).values().toArray(), new Integer[] {  });
        assertEquals(r.third().toArray(), new Integer[] { 102, 111, 111, 98, 97, 114, 0 });
    }
}

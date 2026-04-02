package io.kaitai.struct.spec;

import io.kaitai.struct.ArraySpan;
import io.kaitai.struct.PositionInfo;
import io.kaitai.struct.Span;
import io.kaitai.struct.testformats.Debug0;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertFalse;
import static org.testng.Assert.assertTrue;

public class TestDebug0 extends CommonSpec {
    @Test
    public void testDebug0() throws Exception {
        Debug0 r = Debug0.fromFile(SRC_DIR + "fixed_struct.bin");
        r._read();

        assertEquals(r.one(), 80);
        assertEquals(r.arrayOfInts().toArray(), new Integer[] { 65, 67, 75 });
        assertEquals(r._unnamed2(), 45);

        assertEquals(Debug0._seqFields, new String[] { "one", "arrayOfInts", "_unnamed2" });

        assertTrue(r instanceof PositionInfo, "Structure in debug mode should implement PositionInfo");
        assertEquals(r._spans().size(), 3, "Position information should exists for each field");

        final Span oneSpan = r._spans().get("one");
        assertFalse(oneSpan instanceof ArraySpan, "Non-Array span shouldn't be instanceof ArraySpan");
        assertEquals(oneSpan.start, 0);
        assertEquals(oneSpan.end,   1);

        final Span arraySpan = r._spans().get("arrayOfInts");
        assertEquals(arraySpan.start, 1);
        assertEquals(arraySpan.end, 4);

        assertTrue(arraySpan instanceof ArraySpan, "Array span should be instanceof ArraySpan");

        final ArraySpan array = (ArraySpan)arraySpan;
        assertEquals(array.items.size(), 3);

        final long[] starts = { 1, 2, 3 };
        final long[] ends   = { 2, 3, 4 };
        int i = 0;
        for (final Span span : array.items) {
            assertEquals(span.start, starts[i]);
            assertEquals(span.end,   ends[i]);
            ++i;
        }
    }
}

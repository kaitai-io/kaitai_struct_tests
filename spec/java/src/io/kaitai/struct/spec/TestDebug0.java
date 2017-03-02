package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Debug0;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;

import static org.testng.Assert.assertEquals;

public class TestDebug0 extends CommonSpec {
    @Test
    public void testDebug0() throws Exception {
        Debug0 r = Debug0.fromFile(SRC_DIR + "fixed_struct.bin");
        r._read();

        assertEquals(r.one(), 0x50);
        assertEquals(r.arrayOfInts().toArray(), new Integer[] { 0x41, 0x43, 0x4b });

        assertEquals(Debug0._seqFields, new String[] { "one", "arrayOfInts", "_unnamed2" });

        assertEquals(r._attrStart.get("one").intValue(), 0);
        assertEquals(r._attrEnd.get("one").intValue(), 1);
        assertEquals(r._attrStart.get("arrayOfInts").intValue(), 1);
        assertEquals(r._attrEnd.get("arrayOfInts").intValue(), 4);

        assertEquals(r._arrStart.get("arrayOfInts").toArray(), new Integer[] { 1, 2, 3 });
        assertEquals(r._arrEnd.get("arrayOfInts").toArray(), new Integer[] { 2, 3, 4 });
    }
}

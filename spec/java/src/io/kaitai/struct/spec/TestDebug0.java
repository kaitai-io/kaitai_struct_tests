package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Debug0;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.testng.Assert.assertEquals;

public class TestDebug0 extends CommonSpec {
    @Test
    public void testDebug0() throws Exception {
        Debug0 r = Debug0.fromFile(SRC_DIR + "fixed_struct.bin");
        r._read();

        assertEquals(r.one(), 80);
        assertEquals(r.arrayOfInts().toArray(), new Integer[] { 65, 67, 75 });
        assertEquals(r._unnamed2(), 45);

        assertEquals(Debug0._seqFields, new String[] { "one", "arrayOfInts", "_unnamed2" });

        Map<String, Integer> attrStartExpected = new HashMap<>();
        Map<String, Integer> attrEndExpected = new HashMap<>();
        Map<String, List<Integer>> arrStartExpected = new HashMap<>();
        Map<String, List<Integer>> arrEndExpected = new HashMap<>();

        attrStartExpected.put("one", 0);
        attrEndExpected.put("one", 1);

        attrStartExpected.put("arrayOfInts", 1);
        arrStartExpected.put("arrayOfInts", Arrays.asList(1, 2, 3));
        arrEndExpected.put("arrayOfInts", Arrays.asList(2, 3, 4));
        attrEndExpected.put("arrayOfInts", 4);

        attrStartExpected.put("_unnamed2", 4);
        attrEndExpected.put("_unnamed2", 5);

        assertEquals(r._attrStart, attrStartExpected);
        assertEquals(r._attrEnd, attrEndExpected);
        assertEquals(r._arrStart, arrStartExpected);
        assertEquals(r._arrEnd, arrEndExpected);
    }
}

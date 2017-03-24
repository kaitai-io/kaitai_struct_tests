package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrLiterals;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestStrLiterals extends CommonSpec {
    @Test
    public void testStrLiterals() throws Exception {
        StrLiterals r = StrLiterals.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(
                r.complexStr().toCharArray(),
                new char[] { 0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787 }
        );
        assertEquals(
                r.doubleQuotes().toCharArray(),
                new char[] { 34, 34, 34 }
        );
        assertEquals(r.backslashes().toCharArray(),
                new char[] { 92, 92, 92 }
        );
        assertEquals(r.octalEatup().toCharArray(),
                new char[] { 0, 50, 50 }
        );
        assertEquals(r.octalEatup2().toCharArray(),
                new char[] { 2, 50 }
        );
    }
}

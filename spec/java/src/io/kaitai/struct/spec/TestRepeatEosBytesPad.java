// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatEosBytesPad;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestRepeatEosBytesPad extends CommonSpec {

    @Test
    public void testRepeatEosBytesPad() throws Exception {
        RepeatEosBytesPad r = RepeatEosBytesPad.fromFile(SRC_DIR + "repeat_until_process.bin");

        assertIntEquals(r.records().size(), 3);
        assertEquals(r.records().get((int) 0), new byte[] { -24, -70 });
        assertEquals(r.records().get((int) 1), new byte[] { -6, -98, -72 });
        assertEquals(r.records().get((int) 2), new byte[] { -86, 85, 85, 85, 85 });
    }
}
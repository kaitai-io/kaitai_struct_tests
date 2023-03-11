// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatUntilBytes;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestRepeatUntilBytes extends CommonSpec {
    @Test
    public void testRepeatUntilBytes() throws Exception {
        RepeatUntilBytes r = RepeatUntilBytes.fromFile(SRC_DIR + "repeat_until_process.bin");

        assertIntEquals(r.records().size(), 3);
        assertEquals(r.records().get(((int) 0)), new byte[] { -24, -70, -86, -86, -86 });
        assertEquals(r.records().get(((int) 1)), new byte[] { -6, -98, -72, -86, -86 });
        assertEquals(r.records().get(((int) 2)), new byte[] { -86, 85, 85, 85, 85 });
    }
}

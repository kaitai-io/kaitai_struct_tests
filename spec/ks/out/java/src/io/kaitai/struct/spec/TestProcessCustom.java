// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessCustom;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestProcessCustom extends CommonSpec {
    @Test
    public void testProcessCustom() throws Exception {
        ProcessCustom r = ProcessCustom.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1(), new byte[] { 16, -77, -108, -108, -12 });
        assertEquals(r.buf2(), new byte[] { 95, -70, 123, -109, 99, 35, 95 });
        assertEquals(r.buf3(), new byte[] { 41, 51, -79, 56, -79 });
    }
}

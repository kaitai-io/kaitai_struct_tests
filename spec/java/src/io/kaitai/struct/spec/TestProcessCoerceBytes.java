package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessCoerceBytes;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessCoerceBytes extends CommonSpec {
    @Test
    public void testProcessCoerceBytes() throws Exception {
        ProcessCoerceBytes r = ProcessCoerceBytes.fromFile(SRC_DIR + "process_coerce_bytes.bin");

        assertEquals(r.records().get(0).flag(), 0);
        assertEquals(r.records().get(0).buf(), new byte[] { 0x41, 0x41, 0x41, 0x41 });
        assertEquals(r.records().get(1).flag(), 1);
        assertEquals(r.records().get(1).buf(), new byte[] { 0x42, 0x42, 0x42, 0x42 });
    }
}

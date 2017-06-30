package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessCoerceUsertype2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessCoerceUsertype2 extends CommonSpec {
    @Test
    public void testProcessCoerceUsertype2() throws Exception {
        ProcessCoerceUsertype2 r = ProcessCoerceUsertype2.fromFile(SRC_DIR + "process_coerce_bytes.bin");

        assertEquals(r.records().get(0).flag(), 0);
        assertEquals(r.records().get(0).buf().value(), 0x41414141);
        assertEquals(r.records().get(1).flag(), 1);
        assertEquals(r.records().get(1).buf().value(), 0x42424242);
    }
}

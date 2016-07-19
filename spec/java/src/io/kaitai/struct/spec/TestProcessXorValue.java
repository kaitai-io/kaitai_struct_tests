package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessXorValue;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessXorValue extends CommonSpec {
    @Test
    public void testProcessXorValue() throws Exception {
        ProcessXorValue r = ProcessXorValue.fromFile(SRC_DIR + "process_xor_1.bin");

        assertEquals(r.key(), 0xff);
        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }
}

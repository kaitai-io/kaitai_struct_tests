package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessXorConst;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessXorConst extends CommonSpec {
    @Test
    public void testProcessXorConst() throws Exception {
        ProcessXorConst r = ProcessXorConst.fromFile(SRC_DIR + "process_xor_1.bin");

        assertEquals(r.key(), 0xff);
        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }
}

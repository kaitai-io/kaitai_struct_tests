package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessXor4Const;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessXor4Const extends CommonSpec {
    @Test
    public void testProcessXor4Const() throws Exception {
        ProcessXor4Const r = ProcessXor4Const.fromFile(SRC_DIR + "process_xor_4.bin");

        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }
}

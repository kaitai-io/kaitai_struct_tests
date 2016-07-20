package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessXor4Value;
import org.testng.annotations.Test;

import static org.testng.Assert.*;

public class TestProcessXor4Value extends CommonSpec {
    @Test
    public void testProcessXor4Value() throws Exception {
        ProcessXor4Value r = ProcessXor4Value.fromFile(SRC_DIR + "process_xor_4.bin");

        assertEquals(r.key(), new byte[] { (byte) 0xec, (byte) 0xbb, (byte) 0xa3, 0x14 });
        assertEquals(new String(r.buf(), "UTF-8"), "foo bar");
    }
}

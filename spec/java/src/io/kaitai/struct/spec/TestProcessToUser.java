package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessToUser;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessToUser extends CommonSpec {
    @Test
    public void testProcessToUser() throws Exception {
        ProcessToUser r = ProcessToUser.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1().str(), "Hello");
    }
}

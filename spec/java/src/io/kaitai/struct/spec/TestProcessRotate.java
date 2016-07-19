package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessRotate;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessRotate extends CommonSpec {
    @Test
    public void testProcessRotate() throws Exception {
        ProcessRotate r = ProcessRotate.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1(), "Hello".getBytes());
        assertEquals(r.buf2(), "World".getBytes());
        assertEquals(r.buf3(), "There".getBytes());
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestProcessRotate extends CommonSpec {
    @Test
    public void testProcessRotate() throws Exception {
        ProcessRotate r = ProcessRotate.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1(), "Hello".getBytes());
        assertEquals(r.buf2(), "World".getBytes());
        assertEquals(r.buf3(), "There".getBytes());
    }

}

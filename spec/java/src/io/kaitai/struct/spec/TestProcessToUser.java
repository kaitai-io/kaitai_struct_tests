package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestProcessToUser extends CommonSpec {
    @Test
    public void testProcessToUser() throws Exception {
        ProcessToUser r = ProcessToUser.fromFile(SRC_DIR + "process_rotate.bin");

        assertEquals(r.buf1().str(), "Hello");
    }
}
}

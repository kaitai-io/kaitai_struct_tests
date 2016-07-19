package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestZlibWithHeader78 extends CommonSpec {
    @Test
    public void testZlibWithHeader78() throws Exception {
        ZlibWithHeader78 r = ZlibWithHeader78.fromFile(SRC_DIR + "zlib_with_header_78.bin");

        assertEquals(new String(r.data(), Charset.forName("UTF-8")), "a quick brown fox jumps over");
    }

}

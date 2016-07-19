package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestDefaultBigEndian extends CommonSpec {
    @Test
    public void testDefaultBigEndian() throws Exception {
        DefaultBigEndian r = DefaultBigEndian.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.one(), 0x7000000);
    }

}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestStrEos extends CommonSpec {
    @Test
    public void testStrEos() throws Exception {
        StrEos r = StrEos.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.str(), "foo|bar|baz@");
    }

}

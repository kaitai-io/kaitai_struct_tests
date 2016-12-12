package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TypeTernaryOpaque;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestTypeTernaryOpaque extends CommonSpec {
    @Test
    public void testTypeTernaryOpaque() throws Exception {
        TypeTernaryOpaque r = TypeTernaryOpaque.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.dif().s1(), "foo");
        assertEquals(r.dif().s2(), "bar");
        assertEquals(r.dif().s3(), "|baz@");
    }
}

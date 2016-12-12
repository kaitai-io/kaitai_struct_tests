package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TypeTernary;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestTypeTernary extends CommonSpec {
    @Test
    public void testTypeTernary() throws Exception {
        TypeTernary r = TypeTernary.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.dif().value(), 0x65);
    }
}

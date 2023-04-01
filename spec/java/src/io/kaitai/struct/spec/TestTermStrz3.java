// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TermStrz3;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestTermStrz3 extends CommonSpec {
    @Test
    public void testTermStrz3() throws Exception {
        TermStrz3 r = TermStrz3.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.s1(), "foo");
        assertEquals(r.s2(), "|bar|baz");
        assertEquals(r.s3(), "");
    }
}
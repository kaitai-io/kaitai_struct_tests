// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TermStruct;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestTermStruct extends CommonSpec {
    @Test
    public void testTermStruct() throws Exception {
        TermStruct r = TermStruct.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.s1().value(), new byte[] { 102, 111, 111 });
        assertEquals(r.s2().value(), new byte[] { 98, 97, 114 });
        assertEquals(r.s3().value(), new byte[] { 124, 98, 97, 122, 64 });
    }
}
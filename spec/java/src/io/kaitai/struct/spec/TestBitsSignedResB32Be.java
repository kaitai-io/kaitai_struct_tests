// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsSignedResB32Be;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBitsSignedResB32Be extends CommonSpec {
    @Test
    public void testBitsSignedResB32Be() throws Exception {
        BitsSignedResB32Be r = BitsSignedResB32Be.fromFile(SRC_DIR + "bits_shift_by_b32_le.bin");

        assertIntEquals(r.a(), 4294967295L);
    }
}

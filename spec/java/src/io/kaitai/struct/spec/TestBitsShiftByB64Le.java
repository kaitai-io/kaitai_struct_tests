// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsShiftByB64Le;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBitsShiftByB64Le extends CommonSpec {
    @Test
    public void testBitsShiftByB64Le() throws Exception {
        BitsShiftByB64Le r = BitsShiftByB64Le.fromFile(SRC_DIR + "bits_shift_by_b64_le.bin");

        assertIntEquals(r.a(), 0xffffffffffffffffL);
        assertIntEquals(r.b(), 0);
    }
}

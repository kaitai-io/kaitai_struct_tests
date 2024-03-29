// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsSignedShiftB32Le;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBitsSignedShiftB32Le extends CommonSpec {

    @Test
    public void testBitsSignedShiftB32Le() throws Exception {
        BitsSignedShiftB32Le r = BitsSignedShiftB32Le.fromFile(SRC_DIR + "bits_signed_shift_b32_le.bin");

        assertIntEquals(r.a(), 0);
        assertIntEquals(r.b(), 255);
    }
}

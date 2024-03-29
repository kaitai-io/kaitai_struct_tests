// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsUnalignedB64Le;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBitsUnalignedB64Le extends CommonSpec {

    @Test
    public void testBitsUnalignedB64Le() throws Exception {
        BitsUnalignedB64Le r = BitsUnalignedB64Le.fromFile(SRC_DIR + "process_xor_4.bin");

        assertIntEquals(r.a(), false);
        assertIntEquals(r.b(), 1902324737369038326L);
        assertIntEquals(r.c(), 71);
    }
}

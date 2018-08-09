// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DefaultEndianMod;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestDefaultEndianMod extends CommonSpec {
    @Test
    public void testDefaultEndianMod() throws Exception {
        DefaultEndianMod r = DefaultEndianMod.fromFile(SRC_DIR + "fixed_struct.bin");

        assertIntEquals(r.main().one(), 1262698832);
        assertIntEquals(r.main().nest().two(), -52947);
        assertIntEquals(r.main().nestBe().two(), 1346454347);
    }
}

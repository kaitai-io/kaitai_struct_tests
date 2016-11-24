package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DefaultEndianMod;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDefaultEndianMod extends CommonSpec {
    @Test
    public void testDefaultEndianMod() throws Exception {
        DefaultEndianMod r = DefaultEndianMod.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.main().one(), 0x4b434150);
        assertEquals(r.main().nest().two(), -52947);
        assertEquals(r.main().nestBe().two(), 0x5041434b);
    }
}

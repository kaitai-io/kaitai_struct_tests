package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RecursiveOne;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRecursiveOne extends CommonSpec {
    @Test
    public void testRecursiveOne() throws Exception {
        RecursiveOne r = RecursiveOne.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
        RecursiveOne rec1 = (RecursiveOne) r.next();
        assertEquals(rec1.one(), 0x41);
        RecursiveOne rec2 = (RecursiveOne) rec1.next();
        assertEquals(rec2.one(), 0x43);
        RecursiveOne.Fini rec3 = (RecursiveOne.Fini) rec2.next();
        assertEquals(rec3.finisher(), 0x2d4b);
    }
}

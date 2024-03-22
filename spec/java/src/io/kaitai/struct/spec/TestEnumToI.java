// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumToI;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestEnumToI extends CommonSpec {
    @Test
    public void testEnumToI() throws Exception {
        EnumToI r = EnumToI.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.pet1(), EnumToI.Animal.CAT);
        assertEquals(r.pet2(), EnumToI.Animal.CHICKEN);
        assertIntEquals(r.pet1I(), 7);
        assertEquals(r.pet1IToS(), "7");
        assertIntEquals(r.pet1Mod(), 32775);
        assertIntEquals(r.oneLtTwo(), true);
        assertIntEquals(r.pet1EqInt(), true);
        assertIntEquals(r.pet2EqInt(), false);
    }
}

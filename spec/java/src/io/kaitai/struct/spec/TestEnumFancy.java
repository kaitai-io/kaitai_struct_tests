package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumFancy;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestEnumFancy extends CommonSpec {
    @Test
    public void testEnumFancy() throws Exception {
        EnumFancy r = EnumFancy.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.pet1(), EnumFancy.Animal.CAT);
        assertEquals(r.pet2(), EnumFancy.Animal.CHICKEN);
    }
}

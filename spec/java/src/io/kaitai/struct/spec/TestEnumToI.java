package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumToI;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestEnumToI extends CommonSpec {
    @Test
    public void testEnumToI() throws Exception {
        EnumToI r = EnumToI.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.pet1(), EnumToI.Animal.CAT);
        assertEquals(r.pet2(), EnumToI.Animal.CHICKEN);

        assertEquals(r.pet1I().intValue(), 7);
        assertEquals(r.oneLtTwo().booleanValue(), true);
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumInvalid;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestEnumInvalid extends CommonSpec {
    @Test
    public void testEnumInvalid() throws Exception {
        EnumInvalid r = EnumInvalid.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.pet1(), EnumInvalid.Animal.DOG);
        assertEquals(r.pet2(), null);
    }
}

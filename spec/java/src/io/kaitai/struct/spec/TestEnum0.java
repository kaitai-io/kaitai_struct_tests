package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestEnum0 extends CommonSpec {
    @Test
    public void testEnum0() throws Exception {
        Enum0 r = Enum0.fromFile(SRC_DIR + "enum_0.bin");

        assertEquals(r.pet1(), Enum0.Animal.CAT);
        assertEquals(r.pet2(), Enum0.Animal.CHICKEN);
    }

}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.Enum0;
import org.testng.annotations.Test;

public class TestEnum0 extends CommonSpec {
    @Test
    public void testEnum0() throws Exception {
        Enum0 r = new Enum0();

        r.setPet1(Enum0.Animal.CAT);
        r.setPet2(Enum0.Animal.CHICKEN);

        assertEqualToFile(r, "enum_0.bin");
    }
}

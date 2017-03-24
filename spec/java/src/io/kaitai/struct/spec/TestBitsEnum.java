package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BitsEnum;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBitsEnum extends CommonSpec {
    @Test
    public void testBitsEnum() throws Exception {
        BitsEnum r = BitsEnum.fromFile(SRC_DIR + "fixed_struct.bin");

        // 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
        assertEquals(r.one(), BitsEnum.Animal.PLATYPUS);
        assertEquals(r.two(), BitsEnum.Animal.HORSE);
        assertEquals(r.three(), BitsEnum.Animal.CAT);
    }
}

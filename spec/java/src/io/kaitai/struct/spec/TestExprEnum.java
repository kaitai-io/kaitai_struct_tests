package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprEnum;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestExprEnum extends CommonSpec {
    @Test
    public void testExprEnum() throws Exception {
        ExprEnum r = ExprEnum.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.constDog(), ExprEnum.Animal.DOG);
        assertEquals(r.derivedBoom(), ExprEnum.Animal.BOOM);
        assertEquals(r.derivedDog(), ExprEnum.Animal.DOG);
    }
}

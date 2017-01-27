package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.TypeIntUnaryOp;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertTrue;

public class TestTypeIntUnaryOp extends CommonSpec {
    @Test
    public void testTypeIntUnaryOp() throws Exception {
        TypeIntUnaryOp r = TypeIntUnaryOp.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.valueS2(), (short) 0x4150);
        assertEquals(r.valueS8(), 0x4150ffff312d4b43L);
        assertTrue(r.unaryS2() instanceof Long);
        assertTrue(r.unaryS8() instanceof Long);
        assertEquals(r.unaryS2().shortValue(), (short) -0x4150);
        assertEquals(r.unaryS8().longValue(), -0x4150ffff312d4b43L);
    }
}

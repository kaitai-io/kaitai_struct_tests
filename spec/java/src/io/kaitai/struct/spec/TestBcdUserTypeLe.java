package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BcdUserTypeLe;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBcdUserTypeLe extends CommonSpec {
    @Test
    public void testBcdUserTypeLe() throws Exception {
        BcdUserTypeLe r = BcdUserTypeLe.fromFile(SRC_DIR + "bcd_user_type_le.bin");

        assertEquals(r.ltr().asInt().intValue(), 12345678);
        assertEquals(r.ltr().asStr(), "12345678");
        assertEquals(r.rtl().asInt().intValue(), 87654321);
        assertEquals(r.rtl().asStr(), "87654321");
        assertEquals(r.leadingZeroLtr().asInt().intValue(), 123456);
        assertEquals(r.leadingZeroLtr().asStr(), "00123456");
    }
}

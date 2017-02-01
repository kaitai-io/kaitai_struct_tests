package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BcdUserTypeBe;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBcdUserTypeBe extends CommonSpec {
    @Test
    public void testBcdUserTypeBe() throws Exception {
        BcdUserTypeBe r = BcdUserTypeBe.fromFile(SRC_DIR + "bcd_user_type_be.bin");

        assertEquals(r.ltr().asInt().intValue(), 12345678);
        assertEquals(r.ltr().asStr(), "12345678");
        assertEquals(r.rtl().asInt().intValue(), 87654321);
        assertEquals(r.rtl().asStr(), "87654321");
        assertEquals(r.leadingZeroLtr().asInt().intValue(), 123456);
        assertEquals(r.leadingZeroLtr().asStr(), "00123456");
    }
}

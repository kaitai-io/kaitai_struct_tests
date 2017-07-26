package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ParamsCallShort;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestParamsCallShort extends CommonSpec {
    @Test
    public void testParamsCallShort() throws Exception {
        ParamsCallShort r = ParamsCallShort.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.buf1().body(), "foo|b");
        assertEquals(r.buf2().body(), "ar|ba");
        assertEquals(r.buf2().trailer().intValue(), 0x7a);
    }
}

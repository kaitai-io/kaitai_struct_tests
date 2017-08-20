package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OpaqueWithParam;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestOpaqueWithParam extends CommonSpec {
    @Test
    public void testOpaqueWithParam() throws Exception {
        OpaqueWithParam r = OpaqueWithParam.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.one().buf(), "foo|b");
        assertEquals(r.one().trailer().intValue(), 0x61);
    }
}

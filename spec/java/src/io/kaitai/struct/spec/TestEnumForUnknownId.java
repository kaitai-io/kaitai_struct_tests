package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumForUnknownId;
import org.testng.annotations.Test;

import static org.testng.Assert.assertNull;

public class TestEnumForUnknownId extends CommonSpec {
    @Test
    public void testEnumForUnknownId() throws Exception {
        EnumForUnknownId r = EnumForUnknownId.fromFile(SRC_DIR + "fixed_struct.bin");

        assertNull(r.one());
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OptionalId;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestOptionalId extends CommonSpec {
    @Test
    public void testOptionalId() throws Exception {
        OptionalId r = OptionalId.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r._unnamed0(), 0x50);
        assertEquals(r._unnamed1(), 0x41);
        assertEquals(r._unnamed2(), new byte[] { 0x43, 0x4b, 0x2d, 0x31, (byte) 0xff });
    }
}

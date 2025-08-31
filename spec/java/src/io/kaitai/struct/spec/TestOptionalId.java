package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OptionalId;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestOptionalId extends CommonSpec {
    @Test
    public void testOptionalId() throws Exception {
        OptionalId r = OptionalId.fromFile(SRC_DIR + "fixed_struct.bin");

        assertIntEquals(r._unnamed0(), 80);
        assertIntEquals(r._unnamed1(), 65);
        assertEquals(r._unnamed2(), new byte[] { 67, 75, 45, 49, -1 });
    }
}

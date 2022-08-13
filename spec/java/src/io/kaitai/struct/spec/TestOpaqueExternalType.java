package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OpaqueExternalType;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestOpaqueExternalType extends CommonSpec {

    @Test
    public void testOpaqueExternalType() throws Exception {
        OpaqueExternalType r = OpaqueExternalType.fromFile(SRC_DIR + "term_strz.bin");

        assertIntEquals(r.hw().one(), 102);
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OpaqueExternalType;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestOpaqueExternalType extends CommonSpec {
    @Test
    public void testOpaqueExternalType() throws Exception {
        OpaqueExternalType r = OpaqueExternalType.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.one().s1(), "foo");
        assertEquals(r.one().s2(), "bar");
        assertEquals(r.one().s3(), "|baz@");
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.OpaqueExternalType02Parent;
import io.kaitai.struct.testformats.OpaqueExternalType02Child;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestOpaqueExternalType02Parent extends CommonSpec {

    @Test
    public void testOpaqueExternalType02Parent() throws Exception {
        OpaqueExternalType02Parent r = OpaqueExternalType02Parent.fromFile(SRC_DIR + "term_strz.bin");

        assertEquals(r.parent().child().s1(), "foo");
        assertEquals(r.parent().child().s2(), "bar");
        assertEquals(r.parent().child().s3().s3(), "|baz@");
    }
}

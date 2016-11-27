package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.IfInstances;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertNull;

public class TestIfInstances extends CommonSpec {
    @Test
    public void testIfInstances() throws Exception {
        IfInstances r = IfInstances.fromFile(SRC_DIR + "fixed_struct.bin");

        assertNull(r.neverHappens());
    }
}

// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.InstanceIoUser;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestInstanceIoUser extends CommonSpec {
    @Test
    public void testInstanceIoUser() throws Exception {
        InstanceIoUser r = InstanceIoUser.fromFile(SRC_DIR + "instance_io.bin");

        assertIntEquals(r.qtyEntries(), 3);
        assertEquals(r.entries().get(((int) 0)).name(), "the");
        assertEquals(r.entries().get(((int) 1)).name(), "rainy");
        assertEquals(r.entries().get(((int) 2)).name(), "day it is");
    }
}

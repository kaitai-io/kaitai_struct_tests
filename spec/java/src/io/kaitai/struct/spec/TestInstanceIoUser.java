package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.InstanceIoUser;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestInstanceIoUser extends CommonSpec {
    @Test
    public void testInstanceIoUser() throws Exception {
        InstanceIoUser r = InstanceIoUser.fromFile(SRC_DIR + "instance_io.bin");

        assertEquals(r.qtyEntries(), 3);

        assertEquals(r.entries().get(0).name(), "the");
        assertEquals(r.entries().get(1).name(), "rainy");
        assertEquals(r.entries().get(2).name(), "day it is");
    }
}

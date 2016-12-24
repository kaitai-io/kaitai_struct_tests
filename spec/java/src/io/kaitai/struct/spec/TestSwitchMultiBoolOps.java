package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchMultiBoolOps;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchMultiBoolOps extends CommonSpec {
    @Test
    public void testSwitchMultiBoolOps() throws Exception {
        SwitchMultiBoolOps r = SwitchMultiBoolOps.fromFile(SRC_DIR + "switch_integers.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), 1);
        assertEquals(r.opcodes().get(0).body(), 7);

        assertEquals(r.opcodes().get(1).code(), 2);
        assertEquals(r.opcodes().get(1).body(), 0x4040);

        assertEquals(r.opcodes().get(2).code(), 4);
        assertEquals(r.opcodes().get(2).body(), 4919);

        assertEquals(r.opcodes().get(3).code(), 8);
        assertEquals(r.opcodes().get(3).body(), 4919);
    }
}

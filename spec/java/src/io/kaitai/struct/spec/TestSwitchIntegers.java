package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchIntegers;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchIntegers extends CommonSpec {
    @Test
    public void testSwitchIntegers() throws Exception {
        SwitchIntegers r = SwitchIntegers.fromFile(SRC_DIR + "switch_integers.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), 1);
        assertEquals(r.opcodes().get(0).body().longValue(), 7);

        assertEquals(r.opcodes().get(1).code(), 2);
        assertEquals(r.opcodes().get(1).body().longValue(), 0x4040);

        assertEquals(r.opcodes().get(2).code(), 4);
        assertEquals(r.opcodes().get(2).body().longValue(), 4919);

        assertEquals(r.opcodes().get(3).code(), 8);
        assertEquals(r.opcodes().get(3).body().longValue(), 4919);
    }
}

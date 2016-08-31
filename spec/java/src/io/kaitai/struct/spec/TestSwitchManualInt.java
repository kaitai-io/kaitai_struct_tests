package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualInt.Opcode.*;
import io.kaitai.struct.testformats.SwitchManualInt;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualInt extends CommonSpec {
    @Test
    public void testSwitchManualInt() throws Exception {
        SwitchManualInt r = SwitchManualInt.fromFile(SRC_DIR + "switch_opcodes.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), 83);
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foobar");

        assertEquals(r.opcodes().get(1).code(), 73);
        assertEquals(((Intval) r.opcodes().get(1).body()).value(), 0x42);

        assertEquals(r.opcodes().get(2).code(), 73);
        assertEquals(((Intval) r.opcodes().get(2).body()).value(), 0x37);

        assertEquals(r.opcodes().get(3).code(), 83);
        assertEquals(((Strval) r.opcodes().get(3).body()).value(), "");
    }
}

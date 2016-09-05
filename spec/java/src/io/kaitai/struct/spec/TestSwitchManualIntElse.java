package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualIntElse;
import io.kaitai.struct.testformats.SwitchManualIntElse.Opcode.*;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualIntElse extends CommonSpec {
    @Test
    public void testSwitchManualIntElse() throws Exception {
        SwitchManualIntElse r = SwitchManualIntElse.fromFile(SRC_DIR + "switch_opcodes2.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), 83);
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foo");

        assertEquals(r.opcodes().get(1).code(), 88);
        assertEquals(((Noneval) r.opcodes().get(1).body()).filler(), 0x42);

        assertEquals(r.opcodes().get(2).code(), 89);
        assertEquals(((Noneval) r.opcodes().get(2).body()).filler(), 0xcafe);

        assertEquals(r.opcodes().get(3).code(), 73);
        assertEquals(((Intval) r.opcodes().get(3).body()).value(), 7);
    }
}

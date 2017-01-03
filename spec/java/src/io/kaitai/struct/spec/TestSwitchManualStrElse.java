package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualStrElse;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualStrElse extends CommonSpec {
    @Test
    public void testSwitchManualStrElse() throws Exception {
        SwitchManualStrElse r = SwitchManualStrElse.fromFile(SRC_DIR + "switch_opcodes2.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), "S");
        assertEquals(((SwitchManualStrElse.Opcode.Strval) r.opcodes().get(0).body()).value(), "foo");

        assertEquals(r.opcodes().get(1).code(), "X");
        assertEquals(((SwitchManualStrElse.Opcode.Noneval) r.opcodes().get(1).body()).filler(), 0x42);

        assertEquals(r.opcodes().get(2).code(), "Y");
        assertEquals(((SwitchManualStrElse.Opcode.Noneval) r.opcodes().get(2).body()).filler(), 0xcafe);

        assertEquals(r.opcodes().get(3).code(), "I");
        assertEquals(((SwitchManualStrElse.Opcode.Intval) r.opcodes().get(3).body()).value(), 7);
    }
}

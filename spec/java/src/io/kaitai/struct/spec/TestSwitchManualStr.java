package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualStr;
import io.kaitai.struct.testformats.SwitchManualStr.Opcode.Intval;
import io.kaitai.struct.testformats.SwitchManualStr.Opcode.Strval;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualStr extends CommonSpec {
    @Test
    public void testSwitchManualStr() throws Exception {
        SwitchManualStr r = SwitchManualStr.fromFile(SRC_DIR + "switch_opcodes.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), "S");
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foobar");

        assertEquals(r.opcodes().get(1).code(), "I");
        assertEquals(((Intval) r.opcodes().get(1).body()).value(), 0x42);

        assertEquals(r.opcodes().get(2).code(), "I");
        assertEquals(((Intval) r.opcodes().get(2).body()).value(), 0x37);

        assertEquals(r.opcodes().get(3).code(), "S");
        assertEquals(((Strval) r.opcodes().get(3).body()).value(), "");
    }
}

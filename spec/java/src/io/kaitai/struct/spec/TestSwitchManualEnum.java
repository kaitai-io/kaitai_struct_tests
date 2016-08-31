package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnum;
import io.kaitai.struct.testformats.SwitchManualEnum.Opcode.*;

import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualEnum extends CommonSpec {
    @Test
    public void testSwitchManualEnum() throws Exception {
        SwitchManualEnum r = SwitchManualEnum.fromFile(SRC_DIR + "switch_opcodes.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), Code.STRVAL);
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foobar");

        assertEquals(r.opcodes().get(1).code(), Code.INTVAL);
        assertEquals(((Intval) r.opcodes().get(1).body()).value(), 0x42);

        assertEquals(r.opcodes().get(2).code(), Code.INTVAL);
        assertEquals(((Intval) r.opcodes().get(2).body()).value(), 0x37);

        assertEquals(r.opcodes().get(3).code(), Code.STRVAL);
        assertEquals(((Strval) r.opcodes().get(3).body()).value(), "");
    }
}

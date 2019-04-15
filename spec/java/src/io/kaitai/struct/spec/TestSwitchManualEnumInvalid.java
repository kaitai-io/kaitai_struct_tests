package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalid;
import io.kaitai.struct.testformats.SwitchManualEnumInvalid.Opcode.*;

import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualEnumInvalid extends CommonSpec {
    @Test
    public void testSwitchManualEnumInvalid() throws Exception {
        SwitchManualEnumInvalid r = SwitchManualEnumInvalid.fromFile(SRC_DIR + "enum_negative.bin");

        assertEquals(r.opcodes().size(), 2);

        assertEquals(r.opcodes().get(0).code(), CodeEnum.STRVAL);
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foobar");

        assertEquals(r.opcodes().get(1).code(), CodeEnum.INTVAL);
        assertEquals(((Intval) r.opcodes().get(1).body()).value(), 0x42);
    }
}

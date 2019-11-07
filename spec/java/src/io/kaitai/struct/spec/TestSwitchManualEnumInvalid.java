package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalid;
import io.kaitai.struct.testformats.SwitchManualEnumInvalid.Opcode.*;

import org.testng.annotations.Test;

import static org.testng.Assert.*;

public class TestSwitchManualEnumInvalid extends CommonSpec {
    @Test
    public void testSwitchManualEnumInvalid() throws Exception {
        SwitchManualEnumInvalid r = SwitchManualEnumInvalid.fromFile(SRC_DIR + "enum_negative.bin");

        assertEquals(r.opcodes().size(), 2);

        assertNull(r.opcodes().get(0).code());
        assertNull(r.opcodes().get(0).body());

        assertNull(r.opcodes().get(1).code());
        assertNull(r.opcodes().get(1).body());
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalidElse;
import io.kaitai.struct.testformats.SwitchManualEnumInvalidElse.Opcode.*;

import org.testng.annotations.Test;

import static org.testng.Assert.*;

public class TestSwitchManualEnumInvalidElse extends CommonSpec {
    @Test
    public void testSwitchManualEnumInvalidElse() throws Exception {
        SwitchManualEnumInvalidElse r = SwitchManualEnumInvalidElse.fromFile(SRC_DIR + "enum_negative.bin");

        assertEquals(r.opcodes().size(), 2);

        assertNull(r.opcodes().get(0).code());
        assertEquals(((Defval) r.opcodes().get(0).body()).value().intValue(), 123);

        assertNull(r.opcodes().get(1).code());
        assertEquals(((Defval) r.opcodes().get(1).body()).value().intValue(), 123);
    }
}

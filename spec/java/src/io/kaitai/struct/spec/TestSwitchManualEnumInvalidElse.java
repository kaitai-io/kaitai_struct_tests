package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalidElse;
import io.kaitai.struct.testformats.SwitchManualEnumInvalidElse.Opcode.*;

import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestSwitchManualEnumInvalidElse extends CommonSpec {

    @Test
    public void testSwitchManualEnumInvalidElse() throws Exception {
        SwitchManualEnumInvalidElse r = SwitchManualEnumInvalidElse.fromFile(SRC_DIR + "enum_negative.bin");

        assertIntEquals(r.opcodes().size(), 2);

        assertEquals(r.opcodes().get(0).code(), new CodeEnum.Unknown(255));
        assertIntEquals(((Defval) r.opcodes().get(0).body()).value(), 123);

        assertEquals(r.opcodes().get(1).code(), new CodeEnum.Unknown(1));
        assertIntEquals(((Defval) r.opcodes().get(1).body()).value(), 123);
    }
}

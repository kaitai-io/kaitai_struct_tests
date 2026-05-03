package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalid;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestSwitchManualEnumInvalid extends CommonSpec {
    @Test
    public void testSwitchManualEnumInvalid() throws Exception {
        SwitchManualEnumInvalid r = SwitchManualEnumInvalid.fromFile(SRC_DIR + "enum_negative.bin");
        assertIntEquals(r.opcodes().size(), 2);
        assertNull(r.opcodes().get(((int) 0)).code());
        assertNull(r.opcodes().get(((int) 0)).body());
        assertEquals(r.opcodes().get(((int) 1)).code(), SwitchManualEnumInvalid.Opcode.CodeEnum.FOO);
        assertNull(r.opcodes().get(((int) 1)).body());
    }
}

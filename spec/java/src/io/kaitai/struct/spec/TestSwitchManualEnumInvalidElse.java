package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualEnumInvalidElse;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestSwitchManualEnumInvalidElse extends CommonSpec {
    @Test
    public void testSwitchManualEnumInvalidElse() throws Exception {
        SwitchManualEnumInvalidElse r = SwitchManualEnumInvalidElse.fromFile(SRC_DIR + "enum_negative.bin");
        assertIntEquals(r.opcodes().size(), 2);
        assertNull(r.opcodes().get(((int) 0)).code());
        assertIntEquals(((SwitchManualEnumInvalidElse.Opcode.Defval) (r.opcodes().get(((int) 0)).body())).value(), 123);
        assertEquals(r.opcodes().get(((int) 1)).code(), SwitchManualEnumInvalidElse.Opcode.CodeEnum.FOO);
        assertIntEquals(((SwitchManualEnumInvalidElse.Opcode.Defval) (r.opcodes().get(((int) 1)).body())).value(), 123);
    }
}

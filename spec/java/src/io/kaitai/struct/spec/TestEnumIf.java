package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EnumIf;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestEnumIf extends CommonSpec {
    @Test
    public void testEnumIf() throws Exception {
        EnumIf r = EnumIf.fromFile(SRC_DIR + "if_struct.bin");

        assertEquals(r.op1().opcode(), EnumIf.Opcodes.A_STRING);
        assertEquals(r.op1().argStr().str(), "foo");

        assertEquals(r.op2().opcode(), EnumIf.Opcodes.A_TUPLE);
        assertEquals(r.op2().argTuple().num1(), 0x42);
        assertEquals(r.op2().argTuple().num2(), 0x43);

        assertEquals(r.op3().opcode(), EnumIf.Opcodes.A_STRING);
        assertEquals(r.op3().argStr().str(), "bar");
    }
}

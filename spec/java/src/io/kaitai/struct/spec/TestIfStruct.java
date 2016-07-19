package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.IfStruct;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestIfStruct extends CommonSpec {
    @Test
    public void testIfStruct() throws Exception {
        IfStruct r = IfStruct.fromFile(SRC_DIR + "if_struct.bin");

        assertEquals(r.op1().opcode(), 0x53);
        assertEquals(r.op1().argStr().str(), "foo");

        assertEquals(r.op2().opcode(), 0x54);
        assertEquals(r.op2().argTuple().num1(), 0x42);
        assertEquals(r.op2().argTuple().num2(), 0x43);

        assertEquals(r.op3().opcode(), 0x53);
        assertEquals(r.op3().argStr().str(), "bar");
    }
}

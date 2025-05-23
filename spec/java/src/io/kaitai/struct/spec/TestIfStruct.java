// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.IfStruct;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestIfStruct extends CommonSpec {

    @Test
    public void testIfStruct() throws Exception {
        IfStruct r = IfStruct.fromFile(SRC_DIR + "if_struct.bin");
        assertIntEquals(r.op1().opcode(), 83);
        assertNull(r.op1().argTuple());
        assertEquals(r.op1().argStr().str(), "foo");
        assertIntEquals(r.op2().opcode(), 84);
        assertIntEquals(r.op2().argTuple().num1(), 66);
        assertIntEquals(r.op2().argTuple().num2(), 67);
        assertNull(r.op2().argStr());
        assertIntEquals(r.op3().opcode(), 83);
        assertNull(r.op3().argTuple());
        assertEquals(r.op3().argStr().str(), "bar");
    }
}

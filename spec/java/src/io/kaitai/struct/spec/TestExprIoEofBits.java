package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIoEofBits;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestExprIoEofBits extends CommonSpec {
    @Test
    public void testExprIoEofBits() throws Exception {
        ExprIoEofBits r = ExprIoEofBits.fromFile(SRC_DIR + "nav_parent_switch.bin");
        r._read();

        assertIntEquals(r.foo(), 5167);
        assertIntEquals(r.bar(), 15);
        assertNull(r.baz());
        assertEquals(r.align(), new byte[] {  });
        assertNull(r.qux());
    }
}

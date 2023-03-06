package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ExprIoEofBits;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import io.kaitai.struct.KaitaiStream;
public class TestExprIoEofBits extends CommonSpec {

    @Test
    public void testExprIoEofBits() throws Exception {
        ExprIoEofBits r = ExprIoEofBits.fromFile(SRC_DIR + "nav_parent_switch.bin");

        assertThrows(java.nio.BufferUnderflowException.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._read();
            }
        });

        assertIntEquals(r.foo(), 5167);
        assertIntEquals(r.bar(), 15);
        assertEquals(r.assertIoEofBeforeBaz(), new byte[] {  });
    }
}

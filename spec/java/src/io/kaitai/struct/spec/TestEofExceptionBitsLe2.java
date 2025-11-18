package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionBitsLe2;
import org.testng.annotations.Test;
import static org.testng.Assert.*;

import org.testng.Assert.ThrowingRunnable;
public class TestEofExceptionBitsLe2 extends CommonSpec {
    @Test
    public void testEofExceptionBitsLe2() throws Exception {
        EofExceptionBitsLe2 r = EofExceptionBitsLe2.fromFile(SRC_DIR + "nav_parent_switch.bin");
        assertThrowsEofError(new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._read();
            }
        });
        assertIntEquals(r._io().pos(), 1);
        assertIntEquals(r.preBits(), 1);
        assertTrue(r._attrStart.containsKey("failBits"));
        assertIntEquals(r._attrStart.get("failBits"), 1);
        assertIntEquals(r.failBits(), 0); // actually uninitialized, hence 0 in Java
        assertFalse(r._attrEnd.containsKey("failBits"));
    }
}

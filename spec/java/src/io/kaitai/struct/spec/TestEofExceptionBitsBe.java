package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionBitsBe;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestEofExceptionBitsBe extends CommonSpec {
    @Test
    public void testEofExceptionBitsBe() throws Exception {
        EofExceptionBitsBe r = EofExceptionBitsBe.fromFile(SRC_DIR + "nav_parent_switch.bin");
        assertThrowsEofError(new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._read();
            }
        });
        assertIntEquals(r._io().pos(), 1);
        assertIntEquals(r.preBits(), 0);
        assertTrue(r._attrStart.containsKey("failBits"));
        assertIntEquals(r._attrStart.get("failBits"), 1);
        assertIntEquals(r.failBits(), 0); // actually uninitialized, hence 0 in Java
        assertFalse(r._attrEnd.containsKey("failBits"));
    }
}

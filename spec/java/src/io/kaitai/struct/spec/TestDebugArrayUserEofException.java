package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugArrayUserEofException;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import io.kaitai.struct.KaitaiStream;
public class TestDebugArrayUserEofException extends CommonSpec {

    @Test
    public void testDebugArrayUserEofException() throws Exception {
        final DebugArrayUserEofException r = DebugArrayUserEofException.fromFile(SRC_DIR + "nav_parent_codes.bin");

        assertThrows(java.nio.BufferUnderflowException.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                // --debug implies --no-auto-read
                r._read();
            }
        });

        assertIntEquals(r.oneCat().meow(), 3);
        assertIntEquals(r.oneCat().chirp(), 73);
        assertIntEquals(r.arrayOfCats().size(), 3);
        assertIntEquals(r.arrayOfCats().get((int) 0).meow(), 49);
        assertIntEquals(r.arrayOfCats().get((int) 0).chirp(), 50);
        assertIntEquals(r.arrayOfCats().get((int) 1).meow(), 51);
        assertIntEquals(r.arrayOfCats().get((int) 1).chirp(), 66);
        assertIntEquals(r.arrayOfCats().get((int) 2).meow(), 98);
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugArrayUser;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestDebugArrayUser extends CommonSpec {
    @Test
    public void testDebugArrayUser() throws Exception {
        DebugArrayUser r = DebugArrayUser.fromFile(SRC_DIR + "fixed_struct.bin");

        // --debug implies --no-auto-read
        r._read();

        assertIntEquals(r.oneCat().meow(), 80);
        assertIntEquals(r.arrayOfCats().size(), 3);
        assertIntEquals(r.arrayOfCats().get((int) 0).meow(), 65);
        assertIntEquals(r.arrayOfCats().get((int) 1).meow(), 67);
        assertIntEquals(r.arrayOfCats().get((int) 2).meow(), 75);
    }
}

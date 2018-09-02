package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugArrayUser;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDebugArrayUser extends CommonSpec {
    @Test
    public void testDebugArrayUser() throws Exception {
        DebugArrayUser r = DebugArrayUser.fromFile(SRC_DIR + "fixed_struct.bin");
        r._read();

        assertEquals(r.oneCat().meow(), 0x50);
        assertEquals(r.arrayOfCats().get(0).meow(), 0x41);
        assertEquals(r.arrayOfCats().get(1).meow(), 0x43);
        assertEquals(r.arrayOfCats().get(2).meow(), 0x4b);
    }
}

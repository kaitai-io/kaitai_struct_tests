package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugArrayUserCurrentExcluded;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestDebugArrayUserCurrentExcluded extends CommonSpec {

    @Test
    public void testDebugArrayUserCurrentExcluded() throws Exception {
        DebugArrayUserCurrentExcluded r = DebugArrayUserCurrentExcluded.fromFile(SRC_DIR + "term_strz.bin");

        // --debug implies --no-auto-read
        r._read();

        assertEquals(r.arrayOfCats().get((int) 0).meow(), new byte[] { 102, 111, 111 });
        assertEquals(r.arrayOfCats().get((int) 1).meow(), new byte[] { 124, 98 });
        assertEquals(r.arrayOfCats().get((int) 2).meow(), new byte[] { 97 });
    }
}

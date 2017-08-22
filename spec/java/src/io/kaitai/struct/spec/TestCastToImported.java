package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.CastToImported;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestCastToImported extends CommonSpec {
    @Test
    public void testCastToImported() throws Exception {
        CastToImported r = CastToImported.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one().one(), 0x50);
        assertEquals(r.oneCasted().one(), 0x50);
    }
}

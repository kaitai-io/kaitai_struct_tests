package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsAbs;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestImportsAbs extends CommonSpec {
    @Test
    public void testImportsAbs() throws Exception {
        ImportsAbs r = ImportsAbs.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.len().value().intValue(), 80);
        assertEquals(r.body().length, 80);
    }
}

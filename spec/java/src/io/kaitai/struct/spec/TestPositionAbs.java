package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.PositionAbs;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestPositionAbs extends CommonSpec {
    @Test
    public void testPositionAbs() throws Exception {
        PositionAbs r = PositionAbs.fromFile(SRC_DIR + "position_abs.bin");

        assertEquals(r.indexOffset(), 0x20);
        assertEquals(r.index().entry(), "foo");
    }
}

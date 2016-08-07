package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.PositionToEnd;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestPositionToEnd extends CommonSpec {
    @Test
    public void testPositionToEnd() throws Exception {
        PositionToEnd r = PositionToEnd.fromFile(SRC_DIR + "position_to_end.bin");

        assertEquals(r.index().foo(), 0x42);
        assertEquals(r.index().bar(), 0x1234);
    }
}

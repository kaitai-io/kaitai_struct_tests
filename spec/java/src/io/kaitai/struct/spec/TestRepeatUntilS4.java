package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatUntilS4;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatUntilS4 extends CommonSpec {
    @Test
    public void testRepeatUntilS4() throws Exception {
        RepeatUntilS4 r = RepeatUntilS4.fromFile(SRC_DIR + "repeat_until_s4.bin");

        assertEquals(r.entries().toArray(), new Integer[] { 0x42, 0x1337, -251658241, -1 });
        assertEquals(r.afterall(), "foobar");
    }
}

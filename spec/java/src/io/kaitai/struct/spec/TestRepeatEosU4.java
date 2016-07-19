package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatEosU4;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatEosU4 extends CommonSpec {
    @Test
    public void testRepeatEosU4() throws Exception {
        RepeatEosU4 r = RepeatEosU4.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertEquals(r.numbers().toArray(), new long[] { 0, 0x42, 0x42, 0x815 });
    }
}

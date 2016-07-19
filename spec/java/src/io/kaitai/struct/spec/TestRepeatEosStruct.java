package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatEosStruct;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatEosStruct extends CommonSpec {
    @Test
    public void testRepeatEosStruct() throws Exception {
        RepeatEosStruct r = RepeatEosStruct.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertEquals(r.chunks().size(), 2);
        assertEquals(r.chunks().get(0).offset(), 0);
        assertEquals(r.chunks().get(0).len(), 0x42);
        assertEquals(r.chunks().get(1).offset(), 0x42);
        assertEquals(r.chunks().get(1).len(), 0x815);
    }
}

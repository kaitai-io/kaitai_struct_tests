package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatUntilSized;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatUntilSized extends CommonSpec {
    @Test
    public void testRepeatUntilSized() throws Exception {
        RepeatUntilSized r = RepeatUntilSized.fromFile(SRC_DIR + "repeat_until_process.bin");

        assertEquals(r.records().size(), 3);

        assertEquals(r.records().get(0).marker(), 0xe8);
        assertEquals(r.records().get(0).body(), 0xaaaaaabaL);

        assertEquals(r.records().get(1).marker(), 0xfa);
        assertEquals(r.records().get(1).body(), 0xaaaab89eL);

        assertEquals(r.records().get(2).marker(), 0xaa);
        assertEquals(r.records().get(2).body(), 0x55555555L);
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatNStrz;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatNStrz extends CommonSpec {
    @Test
    public void testRepeatNStrz() throws Exception {
        RepeatNStrz r = RepeatNStrz.fromFile(SRC_DIR + "repeat_n_strz.bin");

        assertEquals(r.qty(), 2);
        assertEquals(r.lines().toArray(), new String[] { "foo", "bar" });
    }
}

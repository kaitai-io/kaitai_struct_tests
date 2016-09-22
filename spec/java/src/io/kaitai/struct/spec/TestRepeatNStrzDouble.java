package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatNStrzDouble;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestRepeatNStrzDouble extends CommonSpec {
    @Test
    public void testRepeatNStrzDouble() throws Exception {
        RepeatNStrzDouble r = RepeatNStrzDouble.fromFile(SRC_DIR + "repeat_n_strz.bin");

        assertEquals(r.qty(), 2);
        assertEquals(r.lines1().toArray(), new String[] { "foo" });
        assertEquals(r.lines2().toArray(), new String[] { "bar" });
    }
}

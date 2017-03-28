package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.RepeatNStrz;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;

public class TestRepeatNStrz extends CommonSpec {
    @Test
    public void testRepeatNStrz() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r.setQty(2);
        r.setLines(new ArrayList<>(Arrays.asList("foo", "bar")));

        assertEqualToFile(r, "repeat_n_strz.bin");
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.RepeatEosU4;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;

public class TestRepeatEosU4 extends CommonSpec {
    @Test
    public void testRepeatEosU4() throws Exception {
        RepeatEosU4 r = new RepeatEosU4();
        r.setNumbers(new ArrayList<>(Arrays.asList(0L, 0x42L, 0x42L, 0x815L)));

        assertEqualToFile(r, "repeat_eos_struct.bin");
    }
}

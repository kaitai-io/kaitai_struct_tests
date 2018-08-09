// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatEosU4;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import java.util.ArrayList;
import java.util.Arrays;
public class TestRepeatEosU4 extends CommonSpec {
    @Test
    public void testRepeatEosU4() throws Exception {
        RepeatEosU4 r = RepeatEosU4.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertEquals(r.numbers(), new ArrayList<Integer>(Arrays.asList(0, 66, 66, 2069)));
    }
}

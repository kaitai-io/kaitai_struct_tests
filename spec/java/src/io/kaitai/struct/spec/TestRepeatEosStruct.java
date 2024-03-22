// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.RepeatEosStruct;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestRepeatEosStruct extends CommonSpec {
    @Test
    public void testRepeatEosStruct() throws Exception {
        RepeatEosStruct r = RepeatEosStruct.fromFile(SRC_DIR + "repeat_eos_struct.bin");

        assertIntEquals(r.chunks().size(), 2);
        assertIntEquals(r.chunks().get(((int) 0)).offset(), 0);
        assertIntEquals(r.chunks().get(((int) 0)).len(), 66);
        assertIntEquals(r.chunks().get(((int) 1)).offset(), 66);
        assertIntEquals(r.chunks().get(((int) 1)).len(), 2069);
    }
}

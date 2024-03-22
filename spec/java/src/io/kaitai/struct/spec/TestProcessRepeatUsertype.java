// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessRepeatUsertype;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestProcessRepeatUsertype extends CommonSpec {
    @Test
    public void testProcessRepeatUsertype() throws Exception {
        ProcessRepeatUsertype r = ProcessRepeatUsertype.fromFile(SRC_DIR + "process_xor_4.bin");

        assertIntEquals(r.blocks().get(((int) 0)).a(), -1975704206);
        assertIntEquals(r.blocks().get(((int) 0)).b(), 20);
        assertIntEquals(r.blocks().get(((int) 1)).a(), 279597642);
        assertIntEquals(r.blocks().get(((int) 1)).b(), 68);
    }
}

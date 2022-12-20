// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessRepeatUsertypeDynargRotate;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestProcessRepeatUsertypeDynargRotate extends CommonSpec {

    @Test
    public void testProcessRepeatUsertypeDynargRotate() throws Exception {
        ProcessRepeatUsertypeDynargRotate r = ProcessRepeatUsertypeDynargRotate.fromFile(SRC_DIR + "process_rotate.bin");

        assertIntEquals(r.blocksRol().get(((int) 0)).a(), 25928);
        assertIntEquals(r.blocksRol().get(((int) 1)).a(), 46902);
        assertIntEquals(r.blocksRor().get(((int) 0)).a(), 29295);
        assertIntEquals(r.blocksRor().get(((int) 1)).a(), 16584);
        assertIntEquals(r.blocksRor().get(((int) 2)).a(), 22810);
        assertIntEquals(r.blocksB().dummy(), 178);
        assertIntEquals(r.blocksB().blocksRol0B(), 108);
        assertIntEquals(r.blocksB().blocksRol1B(), 234);
        assertIntEquals(r.blocksB().blocksRor0B(), 108);
        assertIntEquals(r.blocksB().blocksRor1B(), 138);
        assertIntEquals(r.blocksB().blocksRor2B(), 156);
    }
}

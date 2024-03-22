// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ParamsPassArrayIo;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestParamsPassArrayIo extends CommonSpec {
    @Test
    public void testParamsPassArrayIo() throws Exception {
        ParamsPassArrayIo r = ParamsPassArrayIo.fromFile(SRC_DIR + "enum_negative.bin");

        assertIntEquals(r.first().foo(), 255);
        assertEquals(r.one().buf(), new byte[] { 1 });
    }
}

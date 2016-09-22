package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NestedSameName;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNestedSameName extends CommonSpec {
    @Test
    public void testNestedSameName() throws Exception {
        NestedSameName r = NestedSameName.fromFile(SRC_DIR + "repeat_n_struct.bin");

        assertEquals(r.mainData().mainSize(), 2);
        assertEquals(r.mainData().foo().data(), new byte[] { 0x10, 0, 0, 0 });
    }
}

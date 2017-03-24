package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParentFalse2;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNavParentFalse2 extends CommonSpec {
    @Test
    public void testNavParentFalse2() throws Exception {
        NavParentFalse2 r = NavParentFalse2.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.parentless().foo(), 80);
    }
}

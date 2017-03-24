package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParentFalse;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNavParentFalse extends CommonSpec {
    @Test
    public void testNavParentFalse() throws Exception {
        NavParentFalse r = NavParentFalse.fromFile(SRC_DIR + "nav_parent_codes.bin");

        assertEquals(r.childSize(), 3);
        assertEquals(r.elementA().foo().code(), 73);
        assertEquals(r.elementA().foo().more(), new byte[] { 49, 50, 51 });
        assertEquals(r.elementA().bar().foo().code(), 66);
        assertEquals(r.elementB().foo().code(), 98);
    }
}

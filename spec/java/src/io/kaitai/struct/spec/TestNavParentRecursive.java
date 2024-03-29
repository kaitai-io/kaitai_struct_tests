// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParentRecursive;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestNavParentRecursive extends CommonSpec {

    @Test
    public void testNavParentRecursive() throws Exception {
        NavParentRecursive r = NavParentRecursive.fromFile(SRC_DIR + "enum_negative.bin");

        assertIntEquals(r.value(), 255);
        assertIntEquals(r.next().value(), 1);
        assertIntEquals(r.next().parentValue(), 255);
        assertNull(r.next().next());
    }
}

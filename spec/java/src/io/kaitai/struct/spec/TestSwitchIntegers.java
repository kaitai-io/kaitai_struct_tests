// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchIntegers;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestSwitchIntegers extends CommonSpec {
    @Test
    public void testSwitchIntegers() throws Exception {
        SwitchIntegers r = SwitchIntegers.fromFile(SRC_DIR + "switch_integers.bin");

        assertIntEquals(r.opcodes().size(), 4);
        assertIntEquals(r.opcodes().get(((int) 0)).code(), 1);
        assertIntEquals(r.opcodes().get(((int) 0)).body(), 7);
        assertIntEquals(r.opcodes().get(((int) 1)).code(), 2);
        assertIntEquals(r.opcodes().get(((int) 1)).body(), 16448);
        assertIntEquals(r.opcodes().get(((int) 2)).code(), 4);
        assertIntEquals(r.opcodes().get(((int) 2)).body(), 4919);
        assertIntEquals(r.opcodes().get(((int) 3)).code(), 8);
        assertIntEquals(r.opcodes().get(((int) 3)).body(), 4919);
    }
}

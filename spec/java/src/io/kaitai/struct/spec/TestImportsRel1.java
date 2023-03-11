// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsRel1;
import io.kaitai.struct.testformats.Imported1;
import io.kaitai.struct.testformats.Imported2;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestImportsRel1 extends CommonSpec {
    @Test
    public void testImportsRel1() throws Exception {
        ImportsRel1 r = ImportsRel1.fromFile(SRC_DIR + "fixed_struct.bin");

        assertIntEquals(r.one(), 80);
        assertIntEquals(r.two().one(), 65);
        assertIntEquals(r.two().two().one(), 67);
    }
}

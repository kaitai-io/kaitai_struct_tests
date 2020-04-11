package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsAbsRel;
import io.kaitai.struct.testformats.ImportedAndRel;
import io.kaitai.struct.testformats.ImportedRoot;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestImportsAbsRel extends CommonSpec {

    @Test
    public void testImportsAbsRel() throws Exception {
        ImportsAbsRel r = ImportsAbsRel.fromFile(SRC_DIR + "fixed_struct.bin");

        assertIntEquals(r.one(), 80);
        assertIntEquals(r.two().one(), 65);
        assertIntEquals(r.two().two().one(), 67);
    }
}

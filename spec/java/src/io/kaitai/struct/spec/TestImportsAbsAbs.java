package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ImportsAbsAbs;
import io.kaitai.struct.testformats.ImportedAndAbs;
import io.kaitai.struct.testformats.ImportedRoot;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestImportsAbsAbs extends CommonSpec {

    @Test
    public void testImportsAbsAbs() throws Exception {
        ImportsAbsAbs r = ImportsAbsAbs.fromFile(SRC_DIR + "fixed_struct.bin");

        assertIntEquals(r.one(), 80);
        assertIntEquals(r.two().one(), 65);
        assertIntEquals(r.two().two().one(), 67);
    }
}

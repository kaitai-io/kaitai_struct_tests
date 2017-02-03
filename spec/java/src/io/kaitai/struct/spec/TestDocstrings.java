package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.Docstrings;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDocstrings extends CommonSpec {
    @Test
    public void testDocstrings() throws Exception {
        Docstrings r = Docstrings.fromFile(SRC_DIR + "fixed_struct.bin");
    }
}

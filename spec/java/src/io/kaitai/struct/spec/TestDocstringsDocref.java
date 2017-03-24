package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DocstringsDocref;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDocstringsDocref extends CommonSpec {
    @Test
    public void testDocstringsDocref() throws Exception {
        DocstringsDocref r = DocstringsDocref.fromFile(SRC_DIR + "fixed_struct.bin");
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.FixedContents;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestFixedContents extends CommonSpec {
    @Test
    public void testFixedContents() throws Exception {
        FixedContents r = FixedContents.fromFile(SRC_DIR + "fixed_struct.bin");
    }
}

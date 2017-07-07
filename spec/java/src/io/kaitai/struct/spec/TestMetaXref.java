package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.MetaXref;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestMetaXref extends CommonSpec {
    @Test
    public void testMetaXref() throws Exception {
        MetaXref r = MetaXref.fromFile(SRC_DIR + "fixed_struct.bin");
    }
}

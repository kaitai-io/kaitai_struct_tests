package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.InstanceStd;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestInstanceStd extends CommonSpec {
    @Test
    public void testInstanceStd() throws Exception {
        InstanceStd r = InstanceStd.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.header(), "Some ");
    }
}

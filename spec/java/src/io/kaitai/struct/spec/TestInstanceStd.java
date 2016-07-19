package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestInstanceStd extends CommonSpec {
    @Test
    public void testInstanceStd() throws Exception {
        InstanceStd r = InstanceStd.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.header(), "Some ");
    }

}

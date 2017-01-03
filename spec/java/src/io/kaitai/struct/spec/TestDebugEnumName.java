package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugEnumName;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDebugEnumName extends CommonSpec {
    @Test
    public void testDebugEnumName() throws Exception {
        DebugEnumName r = DebugEnumName.fromFile(SRC_DIR + "fixed_struct.bin");
        r._read();

        // this test is meaningful only for languages that have --debug and do
        // not save enum type info
    }
}

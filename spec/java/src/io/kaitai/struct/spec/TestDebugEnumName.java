package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugEnumName;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestDebugEnumName extends CommonSpec {

    @Test
    public void testDebugEnumName() throws Exception {
        DebugEnumName r = DebugEnumName.fromFile(SRC_DIR + "fixed_struct.bin");

        // --debug implies --no-auto-read
        r._read();

        assertEquals(r.one(), DebugEnumName.TestEnum1.ENUM_VALUE_80);
        assertEquals(r.arrayOfInts().get((int) 0), DebugEnumName.TestEnum2.ENUM_VALUE_65);
        assertEquals(r.testType().field1(), DebugEnumName.TestSubtype.InnerEnum1.ENUM_VALUE_67);
        assertEquals(r.testType().instanceField(), DebugEnumName.TestSubtype.InnerEnum2.ENUM_VALUE_11);
    }
}

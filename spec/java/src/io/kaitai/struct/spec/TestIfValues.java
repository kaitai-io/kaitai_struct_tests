package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.IfValues;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertNull;

public class TestIfValues extends CommonSpec {
    @Test
    public void testIfValues() throws Exception {
        IfValues r = IfValues.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.codes().get(0).opcode(), 80);
        assertEquals(r.codes().get(0).halfOpcode().intValue(), 40);
        assertEquals(r.codes().get(1).opcode(), 65);
        assertNull(r.codes().get(1).halfOpcode());
        assertEquals(r.codes().get(2).opcode(), 67);
        assertNull(r.codes().get(2).halfOpcode());
    }
}

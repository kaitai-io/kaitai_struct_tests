package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchBytearray.Opcode.*;
import io.kaitai.struct.testformats.SwitchBytearray;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchBytearray extends CommonSpec {
    @Test
    public void testSwitchBytearray() throws Exception {
        SwitchBytearray r = SwitchBytearray.fromFile(SRC_DIR + "switch_opcodes.bin");

        assertEquals(r.opcodes().size(), 4);

        assertEquals(r.opcodes().get(0).code(), new byte[] { 83 });
        assertEquals(((Strval) r.opcodes().get(0).body()).value(), "foobar");

        assertEquals(r.opcodes().get(1).code(), new byte[] { 73 });
        assertEquals(((Intval) r.opcodes().get(1).body()).value(), 0x42);

        assertEquals(r.opcodes().get(2).code(), new byte[] { 73 });
        assertEquals(((Intval) r.opcodes().get(2).body()).value(), 0x37);

        assertEquals(r.opcodes().get(3).code(), new byte[] { 83 });
        assertEquals(((Strval) r.opcodes().get(3).body()).value(), "");
    }
}

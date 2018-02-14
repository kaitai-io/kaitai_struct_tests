package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessCoerceSwitch;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestProcessCoerceSwitch extends CommonSpec {
    @Test
    public void testProcessCoerceSwitch() throws Exception {
        ProcessCoerceSwitch r = ProcessCoerceSwitch.fromFile(SRC_DIR + "process_coerce_switch.bin");

        assertEquals(r.bufType(), 0);
        assertEquals(r.flag(), 0);
        assertEquals(((ProcessCoerceSwitch.Foo) r.buf()).bar(), new byte[] {0x41, 0x41, 0x41, 0x41});
    }
}

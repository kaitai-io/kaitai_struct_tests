package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestInstanceStdArray extends CommonSpec {
    @Test
    public void testInstanceStdArray() throws Exception {
        InstanceStdArray r = InstanceStdArray.fromFile(SRC_DIR + "instance_std_array.bin");

        assertEquals(r.ofs(), 0x10);
        assertEquals(r.qtyEntries(), 3);
        assertEquals(r.entrySize(), 4);

        assertEquals(r.entries().size(), 3);
        assertEquals(r.entries().get(0), new byte[] { 0x11, 0x11, 0x11, 0x11 });
        assertEquals(r.entries().get(1), new byte[] { 0x22, 0x22, 0x22, 0x22 });
        assertEquals(r.entries().get(2), new byte[] { 0x33, 0x33, 0x33, 0x33 });
    }

}

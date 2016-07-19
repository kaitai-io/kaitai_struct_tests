package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestInstanceUserArray extends CommonSpec {
    @Test
    public void testInstanceUserArray() throws Exception {
        InstanceUserArray r = InstanceUserArray.fromFile(SRC_DIR + "instance_std_array.bin");

        assertEquals(r.ofs(), 0x10);
        assertEquals(r.qtyEntries(), 3);
        assertEquals(r.entrySize(), 4);

        assertEquals(r.userEntries().size(), 3);
        assertEquals(r.userEntries().get(0).word1(), 0x1111);
        assertEquals(r.userEntries().get(0).word2(), 0x1111);
        assertEquals(r.userEntries().get(1).word1(), 0x2222);
        assertEquals(r.userEntries().get(1).word2(), 0x2222);
        assertEquals(r.userEntries().get(2).word1(), 0x3333);
        assertEquals(r.userEntries().get(2).word2(), 0x3333);
    }

}

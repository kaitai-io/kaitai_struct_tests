package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestBufferedStruct extends CommonSpec {
    @Test
    public void testBufferedStruct() throws Exception {
        BufferedStruct r = BufferedStruct.fromFile(SRC_DIR + "buffered_struct.bin");

        assertEquals(r.len1(), 0x10);

        assertEquals(r._raw_block1(), new byte[] {
                0x42, 0, 0, 0,
                0x43, 0, 0, 0,
                -1, -1, -1, -1,
                -1, -1, -1, -1,
        });
        assertEquals(r.block1().number1(), 0x42);
        assertEquals(r.block1().number2(), 0x43);

        assertEquals(r.len2(), 0x8);

        assertEquals(r._raw_block2(), new byte[] {
                0x44, 0, 0, 0,
                0x45, 0, 0, 0,
        });
        assertEquals(r.block2().number1(), 0x44);
        assertEquals(r.block2().number2(), 0x45);

        assertEquals(r.finisher(), 0xee);
    }

}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestRepeatNStruct extends CommonSpec {
    @Test
    public void testRepeatNStruct() throws Exception {
        RepeatNStruct r = RepeatNStruct.fromFile(SRC_DIR + "repeat_n_struct.bin");

        assertEquals(r.qty(), 2);
        assertEquals(r.chunks().get(0).offset(), 0x10);
        assertEquals(r.chunks().get(0).len(), 0x2078);
        assertEquals(r.chunks().get(1).offset(), 0x2088);
        assertEquals(r.chunks().get(1).len(), 0xf);
    }

}

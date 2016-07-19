package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestFixedStruct extends CommonSpec {
    @Test
    public void testFixedStruct() throws Exception {
        FixedStruct r = FixedStruct.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.hdr().uint8(), 255);
        assertEquals(r.hdr().uint16(), 65535);
        assertEquals(r.hdr().uint32(), 4294967295L);
        //assertEquals(r.hdr().uint64(), 18446744073709551615);
        assertEquals(r.hdr().uint64(), 0xFFFFFFFFFFFFFFFFL);

        assertEquals(r.hdr().sint8(), -1);
        assertEquals(r.hdr().sint16(), -1);
        assertEquals(r.hdr().sint32(), -1);
        assertEquals(r.hdr().sint64(), -1);

        assertEquals(r.hdr().uint16le(), 66);
        assertEquals(r.hdr().uint32le(), 66);
        assertEquals(r.hdr().uint64le(), 66);

        assertEquals(r.hdr().sint16le(), -66);
        assertEquals(r.hdr().sint32le(), -66);
        assertEquals(r.hdr().sint64le(), -66);

        assertEquals(r.hdr().uint16be(), 66);
        assertEquals(r.hdr().uint32be(), 66);
        assertEquals(r.hdr().uint64be(), 66);

        assertEquals(r.hdr().sint16be(), -66);
        assertEquals(r.hdr().sint32be(), -66);
        assertEquals(r.hdr().sint64be(), -66);
    }

}

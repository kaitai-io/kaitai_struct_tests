package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.FixedStruct;
import org.testng.annotations.Test;

public class TestFixedStruct extends CommonSpec {
    @Test
    public void testFixedStruct() throws Exception {
        FixedStruct.Header r = new FixedStruct.Header();

        r.setUint8(255);
        r.setUint16(65535);
        r.setUint32(4294967295L);
        //r.setUint64(18446744073709551615);
        r.setUint64(0xFFFFFFFFFFFFFFFFL);

        r.setSint8((byte) -1);
        r.setSint16((short) -1);
        r.setSint32(-1);
        r.setSint64(-1);

        r.setUint16le(66);
        r.setUint32le(66);
        r.setUint64le(66);

        r.setSint16le((short) -66);
        r.setSint32le(-66);
        r.setSint64le(-66);

        r.setUint16be(66);
        r.setUint32be(66);
        r.setUint64be(66);

        r.setSint16be((short) -66);
        r.setSint32be(-66);
        r.setSint64be(-66);
        
        assertEqualToFile(r, "fixed_struct.bin");
    }
}

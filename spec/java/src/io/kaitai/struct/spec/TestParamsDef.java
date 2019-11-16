package io.kaitai.struct.spec;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.testformats.ParamsDef;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestParamsDef extends CommonSpec {
    @Test
    public void testParamsDef() throws Exception {
        KaitaiStream io = new ByteBufferKaitaiStream(SRC_DIR + "term_strz.bin");
        ParamsDef r = new ParamsDef(io, 5, true);

        assertEquals(r.buf(), "foo|b");
        assertEquals(r.trailer().intValue(), 0x61);
    }
}

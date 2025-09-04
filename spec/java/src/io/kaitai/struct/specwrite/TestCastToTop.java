package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.CastToTop;
import org.testng.annotations.Test;

public class TestCastToTop extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return CastToTop.class;
    }

    @Override
    protected String getSrcFilename() {
        return "fixed_struct.bin";
    }

    @Test
    public void testInstDisabled() throws Exception {
        CastToTop r = new CastToTop();
        r.setCode(0x77);

        CastToTop s = new CastToTop(null, r, r._root());
        s.setCode(0xa8);
        // `cast_to_top.ksy` is an infinitely recursive format, so it's necessary to break
        // the infinite recursion by disabling the `header` instance here.
        s.setHeader_Enabled(false);
        s._check();

        r.setHeader(s);
        r._check();

        byte[] output = new byte[2];
        try (KaitaiStream io = new ByteBufferKaitaiStream(output)) {
            r._write(io);
        }
        assertByteArrayEquals(output, new byte[] { 0x77, (byte) 0xa8 });
    }
}

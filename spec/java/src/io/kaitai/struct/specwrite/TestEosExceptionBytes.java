package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EosExceptionBytes;
import org.testng.annotations.Test;
import org.testng.SkipException;

public class TestEosExceptionBytes extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EosExceptionBytes.class;
    }

    @Override
    protected String getSrcFilename() {
        return "term_strz.bin";
    }

    @Override
    @Test
    protected void testReadWriteRoundtrip() throws Exception {
        throw new SkipException("cannot use roundtrip because parsing is expected to fail");
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testEosExceptionBytes() throws Exception {
        EosExceptionBytes r = new EosExceptionBytes();

        EosExceptionBytes.Data data = new EosExceptionBytes.Data(null, r, r._root());
        data.setBuf(new byte[] { 0, -1, -2, -3, -4, -5, -6 });
        data._check();

        r.setEnvelope(data);
        r._check();

        try (KaitaiStream io = new ByteBufferKaitaiStream(12)) {
            r._write(io);
        }
    }
}

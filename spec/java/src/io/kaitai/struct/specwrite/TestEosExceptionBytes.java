package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EosExceptionBytes;
import org.testng.annotations.Test;

public class TestEosExceptionBytes extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
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

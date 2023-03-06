package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EofExceptionBytes;
import org.testng.annotations.Test;

public class TestEofExceptionBytes extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testEofExceptionBytes() throws Exception {
        EofExceptionBytes r = new EofExceptionBytes();
        r.setBuf(new byte[] { 120, 121, 122, 123, 124, 125, 126, 127, -1, -2, -3, -4, -5 });
        r._check();

        try (KaitaiStream io = new ByteBufferKaitaiStream(12)) {
            r._write(io);
        }
    }
}

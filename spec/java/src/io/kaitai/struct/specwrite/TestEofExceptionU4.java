package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EofExceptionU4;
import org.testng.annotations.Test;

public class TestEofExceptionU4 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testEofExceptionU4() throws Exception {
        EofExceptionU4 r = new EofExceptionU4();
        r.setPrebuf(new byte[] { 120, 121, 122, 123, 124, 125, 126, 127, -128 });
        r.setFailInt(3000500200L);
        r._check();

        try (KaitaiStream io = new ByteBufferKaitaiStream(12)) {
            r._write(io);
        }
    }
}

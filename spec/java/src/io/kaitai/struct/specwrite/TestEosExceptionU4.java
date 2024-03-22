package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EosExceptionU4;
import org.testng.annotations.Test;
import org.testng.SkipException;

public class TestEosExceptionU4 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EosExceptionU4.class;
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
    public void testEosExceptionU4() throws Exception {
        EosExceptionU4 r = new EosExceptionU4();

        EosExceptionU4.Data data = new EosExceptionU4.Data(null, r, r._root());
        data.setPrebuf(new byte[] { 0, -1, -2 });
        data.setFailInt(3000500200L);
        data._check();

        r.setEnvelope(data);
        r._check();

        try (KaitaiStream io = new ByteBufferKaitaiStream(12)) {
            r._write(io);
        }
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ExprIoEofBits;
import org.testng.annotations.Test;
import static org.testng.Assert.*;

public class TestExprIoEofBits extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
    }

    @Test
    public void testExprIoEofBits() throws Exception {
        long ioSize;
        try (KaitaiStream io = new ByteBufferKaitaiStream(SRC_DIR + "nav_parent_switch.bin")) {
            ioSize = io.size();
        }

        ExprIoEofBits r = new ExprIoEofBits();
        r.setFoo(5167);
        r.setBar(15L);
        r.setAssertIoEofBeforeBaz(new byte[] { });
        r.setBaz(6);
        r.setAssertIoEofAfterBaz(new byte[8]); // doesn't matter
        r._check();

        KaitaiStream newIo = new ByteBufferKaitaiStream(ioSize);
        r._write_Seq(newIo);

        assertThrows(java.nio.BufferOverflowException.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                newIo.close();
            }
        });
    }
}

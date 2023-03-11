package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ExprIoEofBits;
import org.testng.annotations.Test;
import static org.testng.Assert.*;

import java.io.File;
import java.io.RandomAccessFile;

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
    public void testExprIoEofBitsBB() throws Exception {
        ExprIoEofBits r = getExprIoEofBits();

        KaitaiStream newIo = new ByteBufferKaitaiStream(3);
        r._write_Seq(newIo);

        assertThrowsEofError(new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                newIo.close();
            }
        });
        // shouldn't do anything (especially not throw any exception),
        // because close() is supposed to be idempotent
        newIo.close();
    }

    @Test
    public void testExprIoEofBitsRAF() throws Exception {
        ExprIoEofBits r = getExprIoEofBits();

        File file = new File(SCRATCH_DIR + "specwrite_TestExprIoEofBits.bin");
        RandomAccessFile raf = new RandomAccessFile(file, "rw");
        raf.setLength(3);

        try {
            KaitaiStream newIo = new RandomAccessFileKaitaiStream(raf);
            r._write_Seq(newIo);

            Throwable thr = expectThrowsEofError(new ThrowingRunnable() {
                @Override
                public void run() throws Throwable {
                    newIo.close();
                }
            });
            // shouldn't do anything (especially not throw any exception),
            // because close() is supposed to be idempotent
            newIo.close();
        } finally {
            file.delete();
        }
    }

    protected ExprIoEofBits getExprIoEofBits() {
        ExprIoEofBits r = new ExprIoEofBits();
        r.setFoo(5167);
        r.setBar(15L);
        r.setAssertIoEofBeforeBaz(new byte[] { });
        r.setBaz(6);
        r.setAssertIoEofAfterBaz(new byte[8]); // doesn't matter
        r._check();

        return r;
    }
}

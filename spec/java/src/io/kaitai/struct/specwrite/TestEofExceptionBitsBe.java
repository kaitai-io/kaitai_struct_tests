package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.testwrite.EofExceptionBitsBe;

import java.io.File;
import java.io.RandomAccessFile;

import org.testng.Assert.ThrowingRunnable;
import org.testng.annotations.Test;
import static org.testng.Assert.assertEquals;

public class TestEofExceptionBitsBe extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
    }

    @Test
    public void testEofExceptionBitsBeBB() throws Exception {
        EofExceptionBitsBe r = getEofExceptionBitsBe();

        try (KaitaiStream io = new ByteBufferKaitaiStream(3)) {
            assertThrowsEofError(new ThrowingRunnable() {
                @Override
                public void run() throws Throwable {
                    r._write_Seq(io);
                }
            });
            assertIntEquals(io.pos(), 1);
        }
    }

    @Test
    public void testEofExceptionBitsBeRAF() throws Exception {
        EofExceptionBitsBe r = getEofExceptionBitsBe();

        File file = new File(SCRATCH_DIR + "specwrite_TestEofExceptionBitsBe.bin");
        RandomAccessFile raf = new RandomAccessFile(file, "rw");
        raf.setLength(3);

        Throwable thr;
        try (KaitaiStream io = new RandomAccessFileKaitaiStream(raf)) {
            thr = expectThrowsEofError(new ThrowingRunnable() {
                @Override
                public void run() throws Throwable {
                    r._write_Seq(io);
                }
            });
            assertIntEquals(io.pos(), 1);
        } finally {
            file.delete();
        }
        assertEquals(thr.getMessage(), "requested to write 3 bytes, but only 2 bytes left in the stream");
    }

    protected EofExceptionBitsBe getEofExceptionBitsBe() {
        EofExceptionBitsBe r = new EofExceptionBitsBe();
        r.setPreBits(0b0000_000);
        r.setFailBits(0b1_01000010_11111111_0);
        r._check();

        return r;
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EofExceptionBytes;

import java.io.File;
import java.io.RandomAccessFile;

import org.testng.Assert.ThrowingRunnable;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.assertEquals;

public class TestEofExceptionBytes extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EofExceptionBytes.class;
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

    @Test
    public void testEofExceptionBytesBB() throws Exception {
        EofExceptionBytes r = getEofExceptionBytes();

        try (KaitaiStream io = new ByteBufferKaitaiStream(12)) {
            assertThrowsEofError(new ThrowingRunnable() {
                @Override
                public void run() throws Throwable {
                    r._write(io);
                }
            });
        }
    }

    @Test
    public void testEofExceptionBytesRAF() throws Exception {
        EofExceptionBytes r = getEofExceptionBytes();

        File file = new File(SCRATCH_DIR + "specwrite_TestEofExceptionBytes.bin");
        RandomAccessFile raf = new RandomAccessFile(file, "rw");
        raf.setLength(12);

        Throwable thr;
        try (KaitaiStream io = new RandomAccessFileKaitaiStream(raf)) {
            thr = expectThrowsEofError(new ThrowingRunnable() {
                @Override
                public void run() throws Throwable {
                    r._write(io);
                }
            });
        } finally {
            file.delete();
        }
        assertEquals(thr.getMessage(), "requested to write 13 bytes, but only 12 bytes left in the stream");
    }

    protected EofExceptionBytes getEofExceptionBytes() {
        EofExceptionBytes r = new EofExceptionBytes();
        r.setBuf(new byte[] { 120, 121, 122, 123, 124, 125, 126, 127, -1, -2, -3, -4, -5 });
        r._check();

        return r;
    }
}

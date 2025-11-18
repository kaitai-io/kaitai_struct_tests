package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.EofExceptionU4;

import java.io.File;
import java.io.RandomAccessFile;

import org.testng.Assert.ThrowingRunnable;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.assertEquals;

public class TestEofExceptionU4 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EofExceptionU4.class;
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
    public void testEofExceptionU4BB() throws Exception {
        EofExceptionU4 r = getEofExceptionU4();

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
    public void testEofExceptionU4RAF() throws Exception {
        EofExceptionU4 r = getEofExceptionU4();

        File file = new File(SCRATCH_DIR + "specwrite_TestEofExceptionU4.bin");
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
        assertEquals(thr.getMessage(), "requested to write 4 bytes, but only 3 bytes left in the stream");
    }

    protected EofExceptionU4 getEofExceptionU4() {
        EofExceptionU4 r = new EofExceptionU4();
        r.setPrebuf(new byte[] { 120, 121, 122, 123, 124, 125, 126, 127, -128 });
        r.setFailInt(3000500200L);
        r._check();

        return r;
    }
}

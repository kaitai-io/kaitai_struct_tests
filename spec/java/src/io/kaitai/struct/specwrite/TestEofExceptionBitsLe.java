package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.testwrite.EofExceptionBitsLe;

import java.io.File;
import java.io.RandomAccessFile;

import org.testng.Assert.ThrowingRunnable;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.assertEquals;

public class TestEofExceptionBitsLe extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EofExceptionBitsLe.class;
    }

    @Override
    protected String getSrcFilename() {
        return "nav_parent_switch.bin";
    }

    @Override
    @Test
    protected void testReadWriteRoundtrip() throws Exception {
        throw new SkipException("cannot use roundtrip because parsing is expected to fail");
    }

    @Test
    public void testEofExceptionBitsLeBB() throws Exception {
        EofExceptionBitsLe r = getEofExceptionBitsLe();

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
    public void testEofExceptionBitsLeRAF() throws Exception {
        EofExceptionBitsLe r = getEofExceptionBitsLe();

        File file = new File(SCRATCH_DIR + "specwrite_TestEofExceptionBitsLe.bin");
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

    protected EofExceptionBitsLe getEofExceptionBitsLe() {
        EofExceptionBitsLe r = new EofExceptionBitsLe();
        r.setPreBits(0b000_0001);
        r.setFailBits(0b0_11111111_01000010_0);
        r._check();

        return r;
    }
}

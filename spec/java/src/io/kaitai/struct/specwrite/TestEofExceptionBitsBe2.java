package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.RandomAccessFileKaitaiStream;
import io.kaitai.struct.testwrite.EofExceptionBitsBe2;

import java.io.File;
import java.io.RandomAccessFile;

import org.testng.Assert.ThrowingRunnable;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.assertEquals;

public class TestEofExceptionBitsBe2 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return EofExceptionBitsBe2.class;
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
    public void testEofExceptionBitsBe2BB() throws Exception {
        EofExceptionBitsBe2 r = getEofExceptionBitsBe2();

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
    public void testEofExceptionBitsBe2RAF() throws Exception {
        EofExceptionBitsBe2 r = getEofExceptionBitsBe2();

        File file = new File(SCRATCH_DIR + "specwrite_TestEofExceptionBitsBe2.bin");
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

    protected EofExceptionBitsBe2 getEofExceptionBitsBe2() {
        EofExceptionBitsBe2 r = new EofExceptionBitsBe2();
        r.setPreBits(0x01);
        r.setFailBits(0b01000010_11111111_0);
        r._check();

        return r;
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ValidFailInst;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.*;

public class TestValidFailInst extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return ValidFailInst.class;
    }

    @Override
    protected String getSrcFilename() {
        return "fixed_struct.bin";
    }

    @Override
    @Test
    protected void testReadWriteRoundtrip() throws Exception {
        throw new SkipException("cannot use roundtrip because parsing is expected to fail");
    }

    @Test
    public void testCheckBadValidNoIo() throws Exception {
        ValidFailInst r = new ValidFailInst();
        r.setA(0x50);
        r.setInst(0x31);
        assertCheckValidFail(r);
    }

    @Test
    public void testCheckBadValidOldIo() throws Exception {
        final ValidFailInst r = ValidFailInst.fromFile(SRC_DIR + getSrcFilename());
        assertThrows(KaitaiStream.ValidationNotEqualError.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._read();
            }
        });
        assertCheckValidFail(r);
    }

    private void assertCheckValidFail(final ValidFailInst r) {
        Throwable thr = expectThrows(KaitaiStream.ValidationNotEqualError.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._check();
            }
        });

        // NB: the error message must not contain the "at pos X: " part because
        // _check() is not supposed to access `_io` at all (even if it happens
        // to be non-`null`, as in this case)
        assertEquals(thr.getMessage(), "/instances/inst: validation failed: not equal, expected 80, but got 49");
    }
}

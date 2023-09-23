package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ValidFailEqInt;
import org.testng.annotations.Test;
import org.testng.SkipException;
import static org.testng.Assert.*;

public class TestValidFailEqInt extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return ValidFailEqInt.class;
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
        ValidFailEqInt r = new ValidFailEqInt();
        r.setFoo(0x50);
        assertCheckValidFail(r);
    }

    @Test
    public void testCheckBadValidOldIo() throws Exception {
        final ValidFailEqInt r = ValidFailEqInt.fromFile(SRC_DIR + getSrcFilename());
        assertThrows(KaitaiStream.ValidationNotEqualError.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._read();
            }
        });
        assertCheckValidFail(r);
    }

    private void assertCheckValidFail(final ValidFailEqInt r) {
        Throwable thr = expectThrows(KaitaiStream.ValidationNotEqualError.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._check();
            }
        });

        // NB: the error message must not contain the "at pos X: " part because
        // _check() is not supposed to access `_io` at all (even if it happens
        // to be non-`null`, as in this case)
        assertEquals(thr.getMessage(), "/seq/0: validation failed: not equal, expected 123, but got 80");
    }
}

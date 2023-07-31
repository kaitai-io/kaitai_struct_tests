package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ProcessRotate;
import org.testng.annotations.Test;

public class TestProcessRotate extends CommonSpec {
    @Test
    public void testProcessRotate() throws Exception {
        // NOTE: unlike the automatic roundtrip test, the `_raw_*` fields are set to `null` in this
        // manual test, so "cheating" by just writing them is impossible

        ProcessRotate r = new ProcessRotate();

        r.setBuf1("Hello".getBytes());
        r.setBuf2("World".getBytes());
        r.setKey(1);
        r.setBuf3("There".getBytes());

        assertEqualToFullFile(r, "process_rotate.bin");
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: buf2,.*")
    public void testCheckSizeMismatchCheck() throws Exception {
        ProcessRotate r = new ProcessRotate();

        r.setBuf1("Hello".getBytes());
        r.setBuf2("Way too long".getBytes());

        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: buf2,.*")
    public void testCheckSizeMismatchCheckOrWrite() throws Exception {
        ProcessRotate r = new ProcessRotate();

        r.setBuf1("Hello".getBytes());
        r.setBuf2("Way too long".getBytes());
        r.setKey(1);
        r.setBuf3("There".getBytes());

        r._check();

        // It would be more user-friendly if the size mismatch of `buf2` was caught already in
        // _check(), as tested by the previous test case checkSizeMismatchCheck() (this is possible
        // because it's known that `process: rol/ror(...)` preserve the input length, so the length
        // of `buf2` is the same as the length of `_raw_buf2` which actually gets written). But if
        // the compiler implementation doesn't want to distinguish among `process` types here, just
        // a check in _write() is probably acceptable as well.
        r._write(new ByteBufferKaitaiStream(
            5 + // buf1
            5 + // buf2
            1 + // key
            5 // buf3
        ));
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return ProcessRotate.class;
    }

    @Override
    protected String getSrcFilename() {
        return "process_rotate.bin";
    }
}

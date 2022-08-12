package io.kaitai.struct.specwrite;

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

        assertEqualToFile(r, "process_rotate.bin");
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: buf2,.*")
    public void checkSizeMismatch() throws Exception {
        ProcessRotate r = new ProcessRotate();

        r.setBuf1("Hello".getBytes());
        r.setBuf2("Way too long".getBytes());

        r._check();
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

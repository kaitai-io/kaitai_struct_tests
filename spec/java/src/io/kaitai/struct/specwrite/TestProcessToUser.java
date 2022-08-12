package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ProcessToUser;
import org.testng.annotations.Test;

public class TestProcessToUser extends CommonSpec {
    @Test
    public void testProcessToUser() throws Exception {
        // NOTE: unlike the automatic roundtrip test, the `_raw_*` fields are set to `null` in this
        // manual test, so "cheating" by just writing them is impossible

        ProcessToUser.JustStr buf1 = new ProcessToUser.JustStr();
        buf1.setStr("Hello");

        ProcessToUser r = new ProcessToUser();
        r.setBuf1(buf1);

        assertEqualToFile(r, "process_rotate.bin");
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return ProcessToUser.class;
    }

    @Override
    protected String getSrcFilename() {
        return "process_rotate.bin";
    }
}

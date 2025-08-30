package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ProcessToUser;

import java.io.FileInputStream;

import org.testng.annotations.Test;

public class TestProcessToUser extends CommonSpec {
    @Test
    public void testProcessToUser() throws Exception {
        // NOTE: unlike the automatic roundtrip test, the `_raw_*` fields are set to `null` in this
        // manual test, so "cheating" by just writing them is impossible

        ProcessToUser r = new ProcessToUser();

        ProcessToUser.JustStr buf1 = new ProcessToUser.JustStr(null, r, r._root());
        buf1.setStr("Hello");
        buf1._check();
        r.setBuf1(buf1);

        r._check();

        byte[] expected = new byte[5];
        try (FileInputStream fis = new FileInputStream(SRC_DIR + "process_rotate.bin")) {
            fis.read(expected);
        }

        byte[] actual = new byte[expected.length];
        try (KaitaiStream io = new ByteBufferKaitaiStream(actual)) {
            r._write(io);
        }

        assertByteArrayEquals(actual, expected);
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

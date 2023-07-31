package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.StrEos;
import org.testng.annotations.Test;

public class TestStrEos extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str, expected: 0, actual: 2")
    public void testCheckLongerIo() throws Exception {
        StrEos r = new StrEos();

        r.setStr("Hello");

        r._check();
        r._write(new ByteBufferKaitaiStream(5 + 2));
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return StrEos.class;
    }

    @Override
    protected String getSrcFilename() {
        return "term_strz.bin";
    }
}

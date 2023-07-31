package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.StrEncodings;
import org.testng.annotations.Test;

public class TestStrEncodings extends CommonSpec {
    @Test(expectedExceptions = NullPointerException.class, expectedExceptionsMessageRegExp = ".*\\bstr2\\b.*")
    public void testCheckNull() throws Exception {
        StrEncodings r = new StrEncodings();

        r.setStr1("woo");
        r.setLenOf1(3);

        r.setLenOf2(15);

        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str2,.*")
    public void testCheckMismatch() throws Exception {
        StrEncodings r = new StrEncodings();

        r.setStr1("Some ASCII");
        r.setStr2("こんにちは");
        r.setStr3("こんにちは");
        r.setStr4("░▒▓");

        // To be auto-derived
        r.setLenOf1(10);
        r.setLenOf2(12); // should be 15
        r.setLenOf3(10);
        r.setLenOf4(3);

        r._check();
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return StrEncodings.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_encodings.bin";
    }
}

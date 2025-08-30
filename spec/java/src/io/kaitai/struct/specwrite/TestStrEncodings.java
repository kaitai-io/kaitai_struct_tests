package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.StrEncodings;
import org.testng.annotations.Test;
import static org.testng.Assert.*;

public class TestStrEncodings extends CommonSpec {
    @Test
    public void testCheckNull() throws Exception {
        final StrEncodings r = new StrEncodings();

        r.setStr1("woo");
        r.setLenOf1(3);

        r.setLenOf2(15);

        Throwable thr = expectThrows(NullPointerException.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._check();
            }
        });
        final String msg = thr.getMessage();
        if (msg != null) {
            final String fieldName = "str2";
            assertTrue(msg.matches(".*\\b" + fieldName + "\\b.*"), "expected the error message to contain '" + fieldName + "', but got [" + msg + "]");
        }
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

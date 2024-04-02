package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrEncodingsEscapingEnc;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import java.nio.charset.IllegalCharsetNameException;
public class TestStrEncodingsEscapingEnc extends CommonSpec {

    @Test
    public void testStrEncodingsEscapingEnc() throws Exception {
        StrEncodingsEscapingEnc r = StrEncodingsEscapingEnc.fromFile(SRC_DIR + "str_encodings.bin");

        assertUnknownEncoding("ASCII\\\\x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str1().v();
            }
        });
        assertUnknownEncoding("UTF-8\\'x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str2().v();
            }
        });
        assertUnknownEncoding("SJIS\\\"x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str3().v();
            }
        });
        assertUnknownEncoding("IBM437\\nx", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str4().v();
            }
        });
    }

    private void assertUnknownEncoding(String expectedEncoding, ThrowingRunnable runnable) {
        IllegalCharsetNameException exc = expectThrows(IllegalCharsetNameException.class, runnable);
        assertEquals(exc.getCharsetName(), expectedEncoding);
    }
}

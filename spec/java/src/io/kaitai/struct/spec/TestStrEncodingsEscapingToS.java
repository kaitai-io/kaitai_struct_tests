package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrEncodingsEscapingToS;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import java.nio.charset.IllegalCharsetNameException;
public class TestStrEncodingsEscapingToS extends CommonSpec {

    @Test
    public void testStrEncodingsEscapingToS() throws Exception {
        final StrEncodingsEscapingToS r = StrEncodingsEscapingToS.fromFile(SRC_DIR + "str_encodings.bin");

        assertUnknownEncoding("ASCII\\\\x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str1();
            }
        });
        assertUnknownEncoding("UTF-8\\'x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str2();
            }
        });
        assertUnknownEncoding("SJIS\\\"x", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str3();
            }
        });
        assertUnknownEncoding("IBM437\\nx", new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r.str4();
            }
        });
    }

    private void assertUnknownEncoding(String expectedEncoding, ThrowingRunnable runnable) {
        IllegalCharsetNameException exc = expectThrows(IllegalCharsetNameException.class, runnable);
        assertEquals(exc.getCharsetName(), expectedEncoding);
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.testwrite.BytesPadTerm;
import org.testng.annotations.Test;

public class TestBytesPadTerm extends CommonSpec {
    @Test
    public void testBytesPadTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();

        r.setStrPad("str1".getBytes());
        r.setStrTerm("str2foo".getBytes());
        r.setStrTermAndPad("str+++3bar+++".getBytes());
        r.setStrTermInclude("str4baz@".getBytes());

        assertEqualToFile(r, "str_pad_term.bin");
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void checkLongerString1() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("1234567890123456789012345".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void checkLongerString2() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void checkLongerString3() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("1234567890123456789".getBytes());
        r.setStrTermAndPad("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkLongerString4() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("1234567890123456789".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkBadTerminator() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("1234567890123456789".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123".getBytes());
        r._check();
    }
}

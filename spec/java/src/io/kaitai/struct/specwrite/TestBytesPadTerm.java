package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.BytesPadTerm;
import org.testng.annotations.Test;

public class TestBytesPadTerm extends CommonSpec {
    @Test
    public void testCheckGoodMaxLengths() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodMinLengths() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("".getBytes());
        r.setStrTerm("".getBytes());
        r.setStrTermAndPad("".getBytes());
        r.setStrTermInclude("@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void testCheckLongerStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("123456789012345678901".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodLastByteStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad(("@@@@@"+"@@@@@"+"@@@@@"+"@@@@?").getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void testCheckBadLastByteStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("123456789012345678@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void testCheckLongerStrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void testCheckBadHasTerminator1StrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("123456789012@4567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void testCheckBadHasTerminator2StrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("1234567890123456789@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void testCheckLongerStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void testCheckBadHasTerminatorStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("@2345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodLastByte1StrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad(("+++++"+"+++++"+"+++++"+"++++").getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodLastByte2StrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad(("+++++"+"+++++"+"+++++"+"++++0").getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void testCheckBadLastByteStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("1234567890123456789+".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckLongerStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEmptyStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckBadNoTerminatorStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodTerminator1StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEarlyTerminator1StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("1@@".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodTerminator2StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("1234567890123456789@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEarlyTerminator2StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123456789012345678@@".getBytes());
        r._check();
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return BytesPadTerm.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_pad_term.bin";
    }
}

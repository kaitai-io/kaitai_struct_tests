package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.BytesPadTerm;
import org.testng.annotations.Test;

public class TestBytesPadTerm extends CommonSpec {
    @Test
    public void checkGoodMaxLengths() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void checkGoodMinLengths() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("".getBytes());
        r.setStrTerm("".getBytes());
        r.setStrTermAndPad("".getBytes());
        r.setStrTermInclude("@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void checkLongerStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("123456789012345678901".getBytes());
        r._check();
    }

    @Test
    public void checkGoodLastByteStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad(("@@@@@"+"@@@@@"+"@@@@@"+"@@@@?").getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void checkBadLastByteStrPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("123456789012345678@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void checkLongerStrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void checkBadHasTerminator1StrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("123456789012@4567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void checkBadHasTerminator2StrTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("1234567890123456789@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void checkLongerStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void checkBadHasTerminatorStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("@2345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void checkGoodLastByte1StrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad(("+++++"+"+++++"+"+++++"+"++++").getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void checkGoodLastByte2StrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad(("+++++"+"+++++"+"+++++"+"++++0").getBytes());
        r.setStrTermInclude("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void checkBadLastByteStrTermAndPad() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("1234567890123456789+".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkLongerStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkEmptyStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkBadNoTerminatorStrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("123".getBytes());
        r._check();
    }

    @Test
    public void checkGoodTerminator1StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("12@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkEarlyTerminator1StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("1@@".getBytes());
        r._check();
    }

    @Test
    public void checkGoodTerminator2StrTermInclude() throws Exception {
        BytesPadTerm r = new BytesPadTerm();
        r.setStrPad("12345678901234567890".getBytes());
        r.setStrTerm("12345678901234567890".getBytes());
        r.setStrTermAndPad("12345678901234567890".getBytes());
        r.setStrTermInclude("1234567890123456789@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void checkEarlyTerminator2StrTermInclude() throws Exception {
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

package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.BytesEosPadTerm;
import org.testng.annotations.Test;

public class TestBytesEosPadTerm extends CommonSpec {
    @Test
    public void testCheckGoodMaxLengths() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "12345678901234567890".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodMinLengths() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "".getBytes());
        setStrTerm(r, "".getBytes());
        setStrTermAndPad(r, "".getBytes());
        setStrTermInclude(r, "@".getBytes());
        check(r);
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testCheckLongerStrPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "123456789012345678901".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodLastByteStrPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, ("@@@@@"+"@@@@@"+"@@@@@"+"@@@@?").getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "12345678901234567890".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_pad,.*")
    public void testCheckBadLastByteStrPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "123456789012345678@".getBytes());
        check(r);
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testCheckLongerStrTerm() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "123456789012345678901".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void testCheckBadHasTerminator1StrTerm() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "123456789012@4567890".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term,.*")
    public void testCheckBadHasTerminator2StrTerm() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "1234567890123456789@".getBytes());
        check(r);
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testCheckLongerStrTermAndPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "123456789012345678901".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void testCheckBadHasTerminatorStrTermAndPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "@2345678901234567890".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodLastByte1StrTermAndPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, ("+++++"+"+++++"+"+++++"+"++++").getBytes());
        setStrTermInclude(r, "12345678901234567890".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodLastByte2StrTermAndPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, ("+++++"+"+++++"+"+++++"+"++++0").getBytes());
        setStrTermInclude(r, "12345678901234567890".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_and_pad,.*")
    public void testCheckBadLastByteStrTermAndPad() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "1234567890123456789+".getBytes());
        check(r);
    }

    @Test(expectedExceptions = java.nio.BufferOverflowException.class)
    public void testCheckLongerStrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "123456789012345678901".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEmptyStrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckBadNoTerminatorStrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "123".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodTerminator1StrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "12@".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEarlyTerminator1StrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "1@@".getBytes());
        check(r);
    }

    @Test
    public void testCheckGoodTerminator2StrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "1234567890123456789@".getBytes());
        check(r);
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: str_term_include,.*")
    public void testCheckEarlyTerminator2StrTermInclude() throws Exception {
        BytesEosPadTerm r = new BytesEosPadTerm();
        setStrPad(r, "12345678901234567890".getBytes());
        setStrTerm(r, "12345678901234567890".getBytes());
        setStrTermAndPad(r, "12345678901234567890".getBytes());
        setStrTermInclude(r, "123456789012345678@@".getBytes());
        check(r);
    }

    private static void setStrPad(BytesEosPadTerm r, byte[] value) {
        BytesEosPadTerm.StrPadType s = new BytesEosPadTerm.StrPadType(null, r, r._root());
        s.setValue(value);
        s._check();

        r.setStrPad(s);
    }

    private static void setStrTerm(BytesEosPadTerm r, byte[] value) {
        BytesEosPadTerm.StrTermType s = new BytesEosPadTerm.StrTermType(null, r, r._root());
        s.setValue(value);
        s._check();

        r.setStrTerm(s);
    }

    private static void setStrTermAndPad(BytesEosPadTerm r, byte[] value) {
        BytesEosPadTerm.StrTermAndPadType s = new BytesEosPadTerm.StrTermAndPadType(null, r, r._root());
        s.setValue(value);
        s._check();

        r.setStrTermAndPad(s);
    }

    private static void setStrTermInclude(BytesEosPadTerm r, byte[] value) {
        BytesEosPadTerm.StrTermIncludeType s = new BytesEosPadTerm.StrTermIncludeType(null, r, r._root());
        s.setValue(value);
        s._check();

        r.setStrTermInclude(s);
    }

    private static void check(BytesEosPadTerm r) {
        final byte[] buf = "12345678901234567890".getBytes();
        if (r.strPad() == null) {
            setStrPad(r, buf);
        }
        if (r.strTerm() == null) {
            setStrTerm(r, buf);
        }
        if (r.strTermAndPad() == null) {
            setStrTermAndPad(r, buf);
        }
        if (r.strTermInclude() == null) {
            setStrTermInclude(r, buf);
        }
        r._check();
        r._write(new ByteBufferKaitaiStream(80));
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return BytesEosPadTerm.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_pad_term.bin";
    }
}

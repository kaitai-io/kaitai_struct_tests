package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.BytesPadTermEqual;
import org.testng.annotations.Test;

public class TestBytesPadTermEqual extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return BytesPadTermEqual.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_pad_term.bin";
    }

    @Test
    public void testCheckGoodMaxLengths() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("12345678901234567890".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodMinLengths() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("".getBytes());
        r.setS2("".getBytes());
        r.setS3("".getBytes());
        r.setS4(".".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s1,.*")
    public void testCheckLongerS1() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s1,.*")
    public void testCheckBadHasTerminator1S1() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("123456789012@4567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s1,.*")
    public void testCheckBadHasTerminator2S1() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("1234567890123456789@".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s2,.*")
    public void testCheckLongerS2() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("123456789012345678901".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodTerminator1S2() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("123456789012345678@".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s2,.*")
    public void testCheckEarlyTerminator1S2() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("1234567890123456@8@".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodLastByteS2() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2(("+++++"+"+++++"+"+++++"+"+++9").getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("12345678901234567890".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s2,.*")
    public void testCheckBadLastByteS2() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("123456789012345678+".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s3,.*")
    public void testCheckLongerS3() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s3,.*")
    public void testCheckBadHasTerminator1S3() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("1234567890123456789+".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s3,.*")
    public void testCheckBadHasTerminator2S3() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567+9".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s4,.*")
    public void testCheckLongerS4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("123456789012345678901".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s4,.*")
    public void testCheckEmptyS4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s4,.*")
    public void testCheckBadNoTerminatorS4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("1234567890123456789".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodTerminator1S4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("123456789012345678.".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s4,.*")
    public void testCheckEarlyTerminator1S4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4(".23456789012345678.".getBytes());
        r._check();
    }

    @Test
    public void testCheckGoodTerminator2S4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("1234567890123456789.".getBytes());
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: s4,.*")
    public void testCheckEarlyTerminator2S4() throws Exception {
        BytesPadTermEqual r = new BytesPadTermEqual();
        r.setS1("12345678901234567890".getBytes());
        r.setS2("12345678901234567890".getBytes());
        r.setS3("12345678901234567890".getBytes());
        r.setS4("123456789012345678..".getBytes());
        r._check();
    }
}

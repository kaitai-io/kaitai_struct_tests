package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.TermBytes4;
import org.testng.annotations.Test;

public class TestTermBytes4 extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: value, expected: 0, actual: 1")
    public void testCheckIsEofAfterIncludeTermMissing() {
        TermBytes4 r = new TermBytes4();

        TermBytes4.S1Type s1 = new TermBytes4.S1Type(null, r, r._root());
        s1.setValue(new byte[] { 102, 111, 111 });
        s1._check();
        r.setS1(s1);

        r.setSkipTerm1(0x7c);

        TermBytes4.S2Type s2 = new TermBytes4.S2Type(null, r, r._root());
        s2.setValue(new byte[] { 98, 97, 114 });
        s2._check();
        r.setS2(s2);

        r.setSkipTerm2(0x7c);

        TermBytes4.S3Type s3 = new TermBytes4.S3Type(null, r, r._root());
        // this field with `include: true` and `eos-error: false` does not end with the terminator,
        // but there is 1 byte left in the stream => consistency error
        s3.setValue(new byte[] { 98, 97 });
        s3._check();
        r.setS3(s3);

        r._check();

        KaitaiStream io = new ByteBufferKaitaiStream(
            3 + // s1
            1 + // skip_term1
            3 + // s2
            1 + // skip_term2
            3 // s3
        );

        r._write(io); // should throw a ConsistencyError
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return TermBytes4.class;
    }

    @Override
    protected String getSrcFilename() {
        return "term_strz.bin";
    }
}

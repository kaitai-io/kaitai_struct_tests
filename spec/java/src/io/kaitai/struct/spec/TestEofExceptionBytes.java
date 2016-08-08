package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionBytes;
import io.kaitai.struct.testformats.StrEncodings;
import org.testng.annotations.Test;

import java.io.EOFException;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertThrows;

public class TestEofExceptionBytes extends CommonSpec {
    @Test(expectedExceptions = EOFException.class)
    public void testEofExceptionBytes() throws Exception {
        EofExceptionBytes r = EofExceptionBytes.fromFile(SRC_DIR + "term_strz.bin");
    }
}

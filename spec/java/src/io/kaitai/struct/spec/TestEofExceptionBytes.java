package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionBytes;
import io.kaitai.struct.testformats.StrEncodings;
import org.testng.annotations.Test;

import java.nio.BufferUnderflowException;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertThrows;

public class TestEofExceptionBytes extends CommonSpec {
    @Test(expectedExceptions = BufferUnderflowException.class)
    public void testEofExceptionBytes() throws Exception {
        EofExceptionBytes r = EofExceptionBytes.fromFile(SRC_DIR + "term_strz.bin");
    }
}

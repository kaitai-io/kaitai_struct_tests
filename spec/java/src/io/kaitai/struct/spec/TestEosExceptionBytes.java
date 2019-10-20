package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EosExceptionBytes;
import io.kaitai.struct.testformats.StrEncodings;
import org.testng.annotations.Test;

import java.nio.BufferUnderflowException;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertThrows;

public class TestEosExceptionBytes extends CommonSpec {
    @Test(expectedExceptions = BufferUnderflowException.class)
    public void testEosExceptionBytes() throws Exception {
        EosExceptionBytes r = EosExceptionBytes.fromFile(SRC_DIR + "term_strz.bin");
    }
}

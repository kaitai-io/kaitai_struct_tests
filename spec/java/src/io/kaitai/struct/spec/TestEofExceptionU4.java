package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionU4;
import org.testng.annotations.Test;

import java.nio.BufferUnderflowException;

import static org.testng.Assert.assertEquals;

public class TestEofExceptionU4 extends CommonSpec {
    @Test(expectedExceptions = BufferUnderflowException.class)
    public void testEofExceptionU4() throws Exception {
        EofExceptionU4 r = EofExceptionU4.fromFile(SRC_DIR + "term_strz.bin");
    }
}

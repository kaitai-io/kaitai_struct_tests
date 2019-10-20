package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EosExceptionU4;
import org.testng.annotations.Test;

import java.nio.BufferUnderflowException;

import static org.testng.Assert.assertEquals;

public class TestEosExceptionU4 extends CommonSpec {
    @Test(expectedExceptions = BufferUnderflowException.class)
    public void testEosExceptionU4() throws Exception {
        EosExceptionU4 r = EosExceptionU4.fromFile(SRC_DIR + "term_strz.bin");
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.testformats.DefaultEndianExprException;
import org.testng.annotations.Test;

import java.nio.BufferUnderflowException;

import static org.testng.Assert.assertEquals;

public class TestDefaultEndianExprException extends CommonSpec {
    @Test(expectedExceptions = KaitaiStream.UndecidedEndiannessError.class)
    public void testDefaultEndianExprException() throws Exception {
        DefaultEndianExprException r = DefaultEndianExprException.fromFile(SRC_DIR + "endian_expr.bin");
    }
}

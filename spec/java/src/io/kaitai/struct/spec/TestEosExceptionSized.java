package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EosExceptionSized;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import io.kaitai.struct.KaitaiStream;
public class TestEosExceptionSized extends CommonSpec {

    @Test(expectedExceptions = IllegalArgumentException.class)
    public void testEosExceptionSized() throws Exception {
        EosExceptionSized r = EosExceptionSized.fromFile(SRC_DIR + "term_strz.bin");
    }
}

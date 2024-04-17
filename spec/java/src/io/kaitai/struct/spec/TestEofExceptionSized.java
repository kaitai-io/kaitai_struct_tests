package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.EofExceptionSized;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import io.kaitai.struct.KaitaiStream;
public class TestEofExceptionSized extends CommonSpec {

    @Test(expectedExceptions = IllegalArgumentException.class)
    public void testEofExceptionSized() throws Exception {
        EofExceptionSized r = EofExceptionSized.fromFile(SRC_DIR + "term_strz.bin");
    }
}

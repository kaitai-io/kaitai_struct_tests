package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrPadTermEmpty;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestStrPadTermEmpty extends CommonSpec {
    @Test
    public void testStrPadTermEmpty() throws Exception {
        StrPadTermEmpty r = StrPadTermEmpty.fromFile(SRC_DIR + "str_pad_term_empty.bin");

        assertEquals(r.strPad(), "");
        assertEquals(r.strTerm(), "");
        assertEquals(r.strTermAndPad(), "");
        assertEquals(r.strTermInclude(), "@");
    }
}

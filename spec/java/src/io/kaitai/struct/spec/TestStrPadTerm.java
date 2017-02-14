package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrPadTerm;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestStrPadTerm extends CommonSpec {
    @Test
    public void testStrPadTerm() throws Exception {
        StrPadTerm r = StrPadTerm.fromFile(SRC_DIR + "str_pad_term.bin");

        assertEquals(r.strPad(), "str1");
        assertEquals(r.strTerm(), "str2foo");
        assertEquals(r.strTermAndPad(), "str+++3bar+++");
        assertEquals(r.strTermInclude(), "str4baz@");
    }
}

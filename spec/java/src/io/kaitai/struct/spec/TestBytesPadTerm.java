package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BytesPadTerm;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestBytesPadTerm extends CommonSpec {
    @Test
    public void testBytesPadTerm() throws Exception {
        BytesPadTerm r = BytesPadTerm.fromFile(SRC_DIR + "str_pad_term.bin");

        assertEquals(r.strPad(), "str1".getBytes());
        assertEquals(r.strTerm(), "str2foo".getBytes());
        assertEquals(r.strTermAndPad(), "str+++3bar+++".getBytes());
        assertEquals(r.strTermInclude(), "str4baz@".getBytes());
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.BytesPadTerm;
import org.testng.annotations.Test;

public class TestBytesPadTerm extends CommonSpec {
    @Test
    public void testBytesPadTerm() throws Exception {
        BytesPadTerm r = new BytesPadTerm(emptyIO());

        r.setStrPad("str1".getBytes());
        r.setStrTerm("str2foo".getBytes());
        r.setStrTermAndPad("str+++3bar+++".getBytes());
        r.setStrTermInclude("str4baz@".getBytes());

        r._write();
        assertEqualToFile(r, "str_pad_term.bin");
    }
}

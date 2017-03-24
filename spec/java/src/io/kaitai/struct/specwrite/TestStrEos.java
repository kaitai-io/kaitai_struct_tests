package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.StrEos;
import org.testng.annotations.Test;

public class TestStrEos extends CommonSpec {
    @Test
    public void testStrEos() throws Exception {
        StrEos r = new StrEos(emptyIO());

        r.setStr("foo|bar|baz@");

        r._write();
        assertEqualToFile(r, "term_strz.bin");
    }
}

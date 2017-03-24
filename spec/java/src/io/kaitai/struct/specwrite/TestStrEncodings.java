package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.StrEncodings;
import org.testng.annotations.Test;

public class TestStrEncodings extends CommonSpec {
    @Test
    public void testStrEncodings() throws Exception {
        StrEncodings r = new StrEncodings(emptyIO());

        r.setStr1("Some ASCII");
        r.setStr2("こんにちは");
        r.setStr3("こんにちは");
        r.setStr4("░▒▓");

        // To be auto-derived
        r.setLenOf1(10);
        r.setLenOf2(15);
        r.setLenOf3(10);
        r.setLenOf4(3);

        r._write();
        assertEqualToFile(r, "str_encodings.bin");
    }
}

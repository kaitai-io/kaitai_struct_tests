package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrEncodings;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestStrEncodings extends CommonSpec {
    @Test
    public void testStrEncodings() throws Exception {
        StrEncodings r = StrEncodings.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.str1(), "Some ASCII");
        assertEquals(r.str2(), "こんにちは");
        assertEquals(r.str3(), "こんにちは");
        assertEquals(r.str4(), "░▒▓");
    }
}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.StrEncodingsDefault;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestStrEncodingsDefault extends CommonSpec {
    @Test
    public void testStrEncodingsDefault() throws Exception {
        StrEncodingsDefault r = StrEncodingsDefault.fromFile(SRC_DIR + "str_encodings.bin");

        assertEquals(r.str1(), "Some ASCII");
        assertEquals(r.rest().str2(), "こんにちは");
        assertEquals(r.rest().str3(), "こんにちは");
        assertEquals(r.rest().str4(), "░▒▓");
    }
}

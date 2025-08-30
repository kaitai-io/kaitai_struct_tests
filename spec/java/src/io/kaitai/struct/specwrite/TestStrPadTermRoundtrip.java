package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.StrPadTermRoundtrip;
import org.testng.annotations.Test;

public class TestStrPadTermRoundtrip extends CommonSpec {
    @Test
    public void testStrPadTermRoundtrip() throws Exception {
        // NOTE: here it makes sense to prefer a manual test over the automatic
        // testReadWriteRoundtrip, because the roundtrip can't always recognize whether the
        // `pad-right` is correct (i.e. whether the serialized field is padded correctly)

        StrPadTermRoundtrip r = new StrPadTermRoundtrip();

        r.setStrPad("str1");
        r.setStrTerm("str2foo");
        r.setStrTermAndPad("str+++3bar+++");
        r.setStrTermInclude("str4baz@");

        assertEqualToFullFile(r, "str_pad_term.bin");
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return StrPadTermRoundtrip.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_pad_term.bin";
    }
}

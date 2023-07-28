package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.BytesPadTermRoundtrip;
import org.testng.annotations.Test;

public class TestBytesPadTermRoundtrip extends CommonSpec {
    @Test
    public void testBytesPadTermRoundtrip() throws Exception {
        // NOTE: here it makes sense to prefer a manual test over the automatic
        // testReadWriteRoundtrip, because the roundtrip can't always recognize whether the
        // `pad-right` is correct (i.e. whether the serialized field is padded correctly)

        BytesPadTermRoundtrip r = new BytesPadTermRoundtrip();

        r.setStrPad("str1".getBytes());
        r.setStrTerm("str2foo".getBytes());
        r.setStrTermAndPad("str+++3bar+++".getBytes());
        r.setStrTermInclude("str4baz@".getBytes());

        assertEqualToFullFile(r, "str_pad_term.bin");
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return BytesPadTermRoundtrip.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_pad_term.bin";
    }
}

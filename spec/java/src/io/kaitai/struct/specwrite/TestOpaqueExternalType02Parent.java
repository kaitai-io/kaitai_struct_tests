package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.OpaqueExternalType02Parent;
import org.testng.annotations.Test;

public class TestOpaqueExternalType02Parent extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return OpaqueExternalType02Parent.class;
    }

    @Override
    protected String getSrcFilename() {
        return "term_strz.bin";
    }
}

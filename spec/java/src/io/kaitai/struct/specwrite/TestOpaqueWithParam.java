package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.OpaqueWithParam;
import org.testng.annotations.Test;

public class TestOpaqueWithParam extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return OpaqueWithParam.class;
    }

    @Override
    protected String getSrcFilename() {
        return "term_strz.bin";
    }
}

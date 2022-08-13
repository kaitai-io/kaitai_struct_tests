package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.ZlibSurrounded;
import org.testng.annotations.Test;

public class TestZlibSurrounded extends CommonSpec {
    // @Override
    // protected Class<? extends ReadWrite> getStructClass() {
    //     return ZlibSurrounded.class;
    // }

    // @Override
    // protected String getSrcFilename() {
    //     return "zlib_surrounded.bin";
    // }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        throw new UnsupportedOperationException();
    }

    @Override
    protected String getSrcFilename() {
        throw new UnsupportedOperationException();
    }
}

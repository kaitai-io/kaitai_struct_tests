// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.FixedStruct;
import org.testng.annotations.Test;

public class TestFixedStruct extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return FixedStruct.class;
    }

    @Override
    protected String getSrcFilename() {
        return "fixed_struct.bin";
    }
}

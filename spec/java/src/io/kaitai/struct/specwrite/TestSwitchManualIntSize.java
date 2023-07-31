package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.SwitchManualIntSize;
import org.testng.annotations.Test;

public class TestSwitchManualIntSize extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: body, expected: 3, actual: 2")
    public void testCheckSwitchBytesSizeMismatch() {
        SwitchManualIntSize.Chunk chunk = new SwitchManualIntSize.Chunk();
        chunk.setCode(0x33);
        chunk.setSize(3);
        // should cause the consistency check to fail because it's 2 bytes, not 3
        chunk.setBody(new byte[] { 0x10, 0x20 });

        chunk._check();
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return SwitchManualIntSize.class;
    }

    @Override
    protected String getSrcFilename() {
        return "switch_tlv.bin";
    }
}

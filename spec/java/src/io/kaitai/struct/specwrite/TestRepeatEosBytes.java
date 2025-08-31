package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.RepeatEosBytes;

import java.util.ArrayList;
import java.util.Arrays;

import org.testng.annotations.Test;

public class TestRepeatEosBytes extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: records, expected: 0, actual: 3")
    public void testCheckLongerIo() throws Exception {
        RepeatEosBytes r = new RepeatEosBytes();

        r.setRecords(new ArrayList<>(
            Arrays.asList(
                new byte[] { -24, -70, -86, -86, -86 },
                new byte[] { -6, -98, -72, -86, -86 },
                new byte[] { -86, 85, 85, 85, 85 }
            )
        ));

        r._check();
        r._write(new ByteBufferKaitaiStream(15 + 3));
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return RepeatEosBytes.class;
    }

    @Override
    protected String getSrcFilename() {
        return "repeat_until_process.bin";
    }
}

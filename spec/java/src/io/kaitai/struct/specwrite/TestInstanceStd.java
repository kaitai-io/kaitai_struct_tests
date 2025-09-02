package io.kaitai.struct.specwrite;

import io.kaitai.struct.ByteBufferKaitaiStream;
import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.InstanceStd;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
import java.lang.reflect.InvocationTargetException;

public class TestInstanceStd extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return InstanceStd.class;
    }

    @Override
    protected String getSrcFilename() {
        return "str_encodings.bin";
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: header,.*")
    public void testCheckShorterHeader() throws Exception {
        InstanceStd r = new InstanceStd();
        r.setHeader("1234");
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: header,.*")
    public void testCheckLongerHeader() throws Exception {
        InstanceStd r = new InstanceStd();
        r.setHeader("123456");
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: header,.*")
    public void testCheckEmptyHeaderViaDump() throws Exception {
        InstanceStd r = new InstanceStd();
        r.setHeader("");
        try {
            // calls all _check() methods
            dumpStruct(r);
        } catch (InvocationTargetException e) {
            if (e.getCause() instanceof Exception) {
                throw ((Exception) e.getCause());
            }
            throw e;
        }
    }

    @Test
    public void testWrite() throws Exception {
        InstanceStd r = new InstanceStd();
        r.setHeader("Hello");

        // see <CommonSpec>.testReadWriteRoundtrip
        Object origDump = dumpStruct(r);

        KaitaiStream io = new ByteBufferKaitaiStream(2 + 5);
        r._write(io);
        io.seek(0);

        InstanceStd newR = new InstanceStd(io);
        newR._read();

        Object newDump = dumpStruct(newR);

        assertEquals(newDump, origDump);
    }
}

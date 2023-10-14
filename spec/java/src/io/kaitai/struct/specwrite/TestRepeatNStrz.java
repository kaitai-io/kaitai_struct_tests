package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.RepeatNStrz;
import org.testng.annotations.Test;
import static org.testng.Assert.*;

import java.util.ArrayList;
import java.util.Arrays;

public class TestRepeatNStrz extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: lines,.*")
    public void testCheckMismatch() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r.setQty(7);
        r.setLines(new ArrayList<>(Arrays.asList("foo", "bar")));

        r._check();
    }

    @Test
    public void testCheckNull() throws Exception {
        final RepeatNStrz r = new RepeatNStrz();
        r.setQty(0);

        Throwable thr = expectThrows(NullPointerException.class, new ThrowingRunnable() {
            @Override
            public void run() throws Throwable {
                r._check();
            }
        });
        final String msg = thr.getMessage();
        if (msg != null) {
            final String fieldName = "lines";
            assertTrue(msg.matches(".*\\b" + fieldName + "\\b.*"), "expected the error message to contain '" + fieldName + "', but got [" + msg + "]");
        }
    }

    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return RepeatNStrz.class;
    }

    @Override
    protected String getSrcFilename() {
        return "repeat_n_strz.bin";
    }
}

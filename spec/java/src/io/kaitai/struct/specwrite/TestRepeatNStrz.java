package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.RepeatNStrz;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;

public class TestRepeatNStrz extends CommonSpec {
    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: lines,.*")
    public void checkMismatch() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r.setQty(7);
        r.setLines(new ArrayList<>(Arrays.asList("foo", "bar")));

        r._check();
    }

    @Test(expectedExceptions = NullPointerException.class, expectedExceptionsMessageRegExp = ".*\\blines\\b.*")
    public void checkNull() throws Exception {
        RepeatNStrz r = new RepeatNStrz();
        r.setQty(0);

        r._check();
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

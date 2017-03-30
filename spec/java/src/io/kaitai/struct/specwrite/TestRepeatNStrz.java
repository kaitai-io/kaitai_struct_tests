package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.testwrite.RepeatNStrz;
import org.testng.annotations.Test;

import java.util.ArrayList;
import java.util.Arrays;

public class TestRepeatNStrz extends CommonSpec {
    @Test
    public void test() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r.setQty(2);
        r.setLines(new ArrayList<>(Arrays.asList("foo", "bar")));

        assertEqualToFile(r, "repeat_n_strz.bin");
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: lines,.*")
    public void checkMismatch() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r.setQty(7);
        r.setLines(new ArrayList<>(Arrays.asList("foo", "bar")));

        r._check();
    }

    @Test(expectedExceptions = NullPointerException.class)
    public void checkNull() throws Exception {
        RepeatNStrz r = new RepeatNStrz();

        r._check();
    }
}

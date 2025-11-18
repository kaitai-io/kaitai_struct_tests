package io.kaitai.struct.specwrite;

import io.kaitai.struct.ConsistencyError;
import io.kaitai.struct.KaitaiStruct.ReadWrite;
import io.kaitai.struct.testwrite.RepeatUntilS4;

import java.util.ArrayList;
import java.util.Arrays;

import org.testng.annotations.Test;

public class TestRepeatUntilS4 extends CommonSpec {
    @Override
    protected Class<? extends ReadWrite> getStructClass() {
        return RepeatUntilS4.class;
    }

    @Override
    protected String getSrcFilename() {
        return "repeat_until_s4.bin";
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: entries,.*")
    public void testCheckBadNoEntries() {
        RepeatUntilS4 r = new RepeatUntilS4();
        r.setEntries(new ArrayList<>(0));
        r._check();
    }

    @Test
    public void testCheckGoodOneEntry() {
        RepeatUntilS4 r = new RepeatUntilS4();
        r.setEntries(new ArrayList<>(Arrays.asList(-1)));
        r.setAfterall("");
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: entries,.*")
    public void testCheckBadEarlyUntilEntry() {
        RepeatUntilS4 r = new RepeatUntilS4();
        r.setEntries(new ArrayList<>(Arrays.asList(-3, 275000, -1, 0, -1)));
        r._check();
    }

    @Test(expectedExceptions = ConsistencyError.class, expectedExceptionsMessageRegExp = "Check failed: entries,.*")
    public void testCheckBadNoUntilEntry() {
        RepeatUntilS4 r = new RepeatUntilS4();
        r.setEntries(new ArrayList<>(Arrays.asList(-3, 275000, -2, 0)));
        r._check();
    }

    @Test
    public void testCheckGoodUntilEntry() {
        RepeatUntilS4 r = new RepeatUntilS4();
        r.setEntries(new ArrayList<>(Arrays.asList(-3, 275000, -2, 0, -1)));
        r.setAfterall("");
        r._check();
    }
}

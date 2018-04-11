package io.kaitai.struct.spec;

import static org.testng.Assert.assertEquals;

public class CommonSpec {
    public String SRC_DIR = "../../src/";

    // Family of assertIntEquals methods helps to compare integer-like values in Java,
    // ignoring whether the value was a boxed type (Integer) or primitive type (int).

    public void assertIntEquals(byte actual, byte expected) {
        assertEquals(actual, expected);
    }

    public void assertIntEquals(short actual, short expected) {
        assertEquals(actual, expected);
    }

    public void assertIntEquals(int actual, int expected) {
        assertEquals(actual, expected);
    }

    public void assertIntEquals(long actual, long expected) {
        assertEquals(actual, expected);
    }

    public void assertIntEquals(Byte actual, byte expected) {
        assertEquals(actual.byteValue(), expected);
    }

    public void assertIntEquals(Short actual, short expected) {
        assertEquals(actual.shortValue(), expected);
    }

    public void assertIntEquals(Integer actual, int expected) {
        assertEquals(actual.intValue(), expected);
    }

    public void assertIntEquals(Long actual, long expected) {
        assertEquals(actual.longValue(), expected);
    }

    public void assertIntEquals(boolean actual, boolean expected) {
        assertEquals(actual, expected);
    }

    public void assertIntEquals(Boolean actual, boolean expected) {
        assertEquals(actual.booleanValue(), expected);
    }
}

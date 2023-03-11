package io.kaitai.struct.spec;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.expectThrows;

import java.io.EOFException;
import org.testng.Assert.ThrowingRunnable;

public class CommonSpec {
    public static String SRC_DIR = "../../src/";
    public static String SCRATCH_DIR = "../../src/scratch/";

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

    protected void assertThrowsEofError(ThrowingRunnable runnable) {
        expectThrowsEofError(runnable);
    }

    protected Throwable expectThrowsEofError(ThrowingRunnable runnable) {
        RuntimeException t = expectThrows(RuntimeException.class, runnable);
        if (t instanceof java.nio.BufferUnderflowException) {
            // OK
        } else if (t instanceof java.nio.BufferOverflowException) {
            // OK
        } else {
            if (t.getCause() instanceof EOFException) {
                // OK
                return t.getCause();
            } else {
                String mismatchMessage;
                if (t.getClass().equals(RuntimeException.class)) {
                    mismatchMessage = String.format(
                        "Expected the cause of %s to be %s, but it was %s",
                        RuntimeException.class.getSimpleName(),
                        EOFException.class.getSimpleName(),
                        t.getCause() == null ? "null" : t.getCause().getClass().getSimpleName()
                    );
                } else {
                    mismatchMessage = String.format(
                        "Expected %s with cause %s, but %s was thrown",
                        RuntimeException.class.getSimpleName(),
                        EOFException.class.getSimpleName(),
                        t.getClass().getSimpleName()
                    );
                }
                throw new AssertionError(mismatchMessage, t.getCause());
            }
        }

        return t;
    }
}

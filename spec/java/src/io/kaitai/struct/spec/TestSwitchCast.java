package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchCast;
import org.testng.Assert;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertThrows;

public class TestSwitchCast extends CommonSpec {
    @Test
    public void testSwitchCast() throws Exception {
        final SwitchCast r = SwitchCast.fromFile(SRC_DIR + "switch_opcodes.bin");

        assertEquals(r.firstObj().value(), "foobar");
        assertEquals(r.secondVal().intValue(), 0x42);

        assertThrows(
                ClassCastException.class,
                new Assert.ThrowingRunnable() {
                    @Override
                    public void run() throws Throwable {
                        r.errCast();
                    }
                }
        );
    }
}

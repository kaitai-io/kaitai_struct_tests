package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchRepeatExpr;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchRepeatExpr extends CommonSpec {
    @Test
    public void testSwitchRepeatExpr() throws Exception {
        SwitchRepeatExpr r = SwitchRepeatExpr.fromFile(SRC_DIR + "switch_tlv.bin");

        assertEquals(r.code(), 0x11);
        assertEquals(r.size(), 9);
        assertEquals(
                ((SwitchRepeatExpr.One) r.body().get(0)).first(),
                "Stuff\0Me\0".getBytes()
        );
    }
}

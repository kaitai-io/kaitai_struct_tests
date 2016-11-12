package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.NavParentSwitch;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestNavParentSwitch extends CommonSpec {
    @Test
    public void testNavParentSwitch() throws Exception {
        NavParentSwitch r = NavParentSwitch.fromFile(SRC_DIR + "nav_parent_switch.bin");

        assertEquals(r.category(), 1);
        NavParentSwitch.Element1 content = (NavParentSwitch.Element1) r.content();
        assertEquals(content.foo(), 0x42);
        assertEquals(content.subelement().bar().intValue(), 0xff);
    }
}

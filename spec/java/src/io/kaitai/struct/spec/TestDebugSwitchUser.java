// runs in debug mode, so the _read() needs to be called manually

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DebugSwitchUser;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestDebugSwitchUser extends CommonSpec {

    @Test
    public void testDebugSwitchUser() throws Exception {
        DebugSwitchUser r = DebugSwitchUser.fromFile(SRC_DIR + "nav_parent_switch.bin");
        r._read();

        assertIntEquals(r.code(), 1);
        assertIntEquals(((DebugSwitchUser.One) (r.data())).val(), -190);
    }
}

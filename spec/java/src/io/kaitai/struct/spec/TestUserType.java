package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.UserType;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestUserType extends CommonSpec {
    @Test
    public void testUserType() throws Exception {
        UserType r = UserType.fromFile(SRC_DIR + "repeat_until_s4.bin");

        assertEquals(r.one().width(), 0x42);
        assertEquals(r.one().height(), 0x1337);
    }
}

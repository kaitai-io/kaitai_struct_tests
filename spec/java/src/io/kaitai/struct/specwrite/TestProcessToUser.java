package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.ProcessToUser;
import org.testng.annotations.Test;

public class TestProcessToUser extends CommonSpec {
    @Test
    public void testProcessToUser() throws Exception {
        ProcessToUser.JustStr buf1 = new ProcessToUser.JustStr();
        buf1.setStr("Hello");

        ProcessToUser r = new ProcessToUser();
        r.setBuf1(buf1);

        assertEqualToFile(r, "process_rotate.bin");
    }
}

package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.ProcessRotate;
import org.testng.annotations.Test;

public class TestProcessRotate extends CommonSpec {
    @Test
    public void testProcessRotate() throws Exception {
        ProcessRotate r = new ProcessRotate(emptyIO());

        r.setBuf1("Hello".getBytes());
        r.setBuf2("World".getBytes());
        r.setKey(1);
        r.setBuf3("There".getBytes());

        r._write();
        assertEqualToFile(r, "process_rotate.bin");
    }
}

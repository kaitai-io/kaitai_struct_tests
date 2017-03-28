package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.HelloWorld;
import org.testng.annotations.Test;

public class TestHelloWorld extends CommonSpec {
    @Test
    public void testHelloWorld() throws Exception {
        HelloWorld r = new HelloWorld();
        r.setOne(0x50);

        assertEqualToFile(r, "fixed_struct.bin");
    }
}

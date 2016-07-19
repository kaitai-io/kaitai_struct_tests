package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.HelloWorld;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestHelloWorld extends CommonSpec {
    @Test
    public void testHelloWorld() throws Exception {
        HelloWorld r = HelloWorld.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.one(), 0x50);
    }

}

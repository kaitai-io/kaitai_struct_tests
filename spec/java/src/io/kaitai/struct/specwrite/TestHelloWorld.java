package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.testwrite.HelloWorld;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestHelloWorld extends CommonSpec {
    @Test
    public void testHelloWorld() throws Exception {
        KaitaiStream io = emptyIO();
        assertEquals(io.pos(), 0);

        HelloWorld r = new HelloWorld(io);
        r.setOne(0x50);

        r._write();
        assertEquals(io.pos(), 1);

        assertEqualToFile(r, "fixed_struct.bin");
    }
}

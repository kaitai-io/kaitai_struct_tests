package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DefaultEndianExprIsLe;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDefaultEndianExprIsLe extends CommonSpec {
    @Test
    public void testDefaultEndianExprIsLe() throws Exception {
        DefaultEndianExprIsLe r = DefaultEndianExprIsLe.fromFile(SRC_DIR + "endian_expr.bin");

        assertEquals(r.docs().get(0).indicator(), new byte[] { 0x49, 0x49 });
        assertEquals(r.docs().get(0).main().someInt(), 0x42);
        assertEquals(r.docs().get(0).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(0).main().someIntLe(), 0x42);

        assertEquals(r.docs().get(1).indicator(), new byte[] { 0x4d, 0x4d });
        assertEquals(r.docs().get(1).main().someInt(), 0x42);
        assertEquals(r.docs().get(1).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(1).main().someIntLe(), 0x42);

        assertEquals(r.docs().get(2).indicator(), new byte[] { 0x58, 0x58 });
        assertEquals(r.docs().get(2).main().someInt(), 0x42);
        assertEquals(r.docs().get(2).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(2).main().someIntLe(), 0x42);
    }
}

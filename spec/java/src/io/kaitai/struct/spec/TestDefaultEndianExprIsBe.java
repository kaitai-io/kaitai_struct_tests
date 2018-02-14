package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DefaultEndianExprIsBe;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDefaultEndianExprIsBe extends CommonSpec {
    @Test
    public void testDefaultEndianExprIsBe() throws Exception {
        DefaultEndianExprIsBe r = DefaultEndianExprIsBe.fromFile(SRC_DIR + "endian_expr.bin");

        // LE
        assertEquals(r.docs().get(0).indicator(), new byte[] { 0x49, 0x49 });
        assertEquals(r.docs().get(0).main().someInt(), 0x42);
        assertEquals(r.docs().get(0).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(0).main().someIntLe(), 0x42);

        assertEquals(r.docs().get(0).main().instInt().intValue(), 0x42);
        assertEquals(r.docs().get(0).main().instSub().foo(), 0x42);

        // BE
        assertEquals(r.docs().get(1).indicator(), new byte[] { 0x4d, 0x4d });
        assertEquals(r.docs().get(1).main().someInt(), 0x42);
        assertEquals(r.docs().get(1).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(1).main().someIntLe(), 0x42);

        assertEquals(r.docs().get(1).main().instInt().intValue(), 0x42000000);
        assertEquals(r.docs().get(1).main().instSub().foo(), 0x42000000);

        // Weird => LE
        assertEquals(r.docs().get(2).indicator(), new byte[] { 0x58, 0x58 });
        assertEquals(r.docs().get(2).main().someInt(), 0x42000000);
        assertEquals(r.docs().get(2).main().someIntBe(), 0x42);
        assertEquals(r.docs().get(2).main().someIntLe(), 0x42);

        assertEquals(r.docs().get(2).main().instInt().intValue(), 0x42);
        assertEquals(r.docs().get(2).main().instSub().foo(), 0x42);
    }
}

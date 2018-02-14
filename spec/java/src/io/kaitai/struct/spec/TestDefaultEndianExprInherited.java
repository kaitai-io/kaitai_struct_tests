package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.DefaultEndianExprInherited;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestDefaultEndianExprInherited extends CommonSpec {
    @Test
    public void testDefaultEndianExprInherited() throws Exception {
        DefaultEndianExprInherited r = DefaultEndianExprInherited.fromFile(SRC_DIR + "endian_expr.bin");

        assertEquals(r.docs().get(0).indicator(), new byte[] { 0x49, 0x49 });
        assertEquals(r.docs().get(0).main().insides().someInt(), 0x42);
        assertEquals(r.docs().get(0).main().insides().more().someInt1(), 0x4200);
        assertEquals(r.docs().get(0).main().insides().more().someInt2(), 0x42);
        assertEquals(r.docs().get(0).main().insides().more().someInst().intValue(), 0x42);

        assertEquals(r.docs().get(1).indicator(), new byte[] { 0x4d, 0x4d });
        assertEquals(r.docs().get(1).main().insides().someInt(), 0x42);
        assertEquals(r.docs().get(1).main().insides().more().someInt1(), 0x42);
        assertEquals(r.docs().get(1).main().insides().more().someInt2(), 0x4200);
        assertEquals(r.docs().get(1).main().insides().more().someInst().intValue(), 0x42000000);

        assertEquals(r.docs().get(2).indicator(), new byte[] { 0x58, 0x58 });
        assertEquals(r.docs().get(2).main().insides().someInt(), 0x42);
        assertEquals(r.docs().get(2).main().insides().more().someInt1(), 0x42);
        assertEquals(r.docs().get(2).main().insides().more().someInt2(), 0x4200);
        assertEquals(r.docs().get(2).main().insides().more().someInst().intValue(), 0x42000000);
    }
}

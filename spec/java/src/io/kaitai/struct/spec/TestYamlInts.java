package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.YamlInts;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestYamlInts extends CommonSpec {
    @Test
    public void testYamlInts() throws Exception {
        YamlInts r = YamlInts.fromFile(SRC_DIR + "fixed_struct.bin");

        assertEquals(r.testU4Dec().intValue(), 0xffffffff);
        assertEquals(r.testU4Hex().intValue(), 0xffffffff);
        assertEquals(r.testU8Dec().longValue(), 0xffffffffffffffffL);
        assertEquals(r.testU8Hex().longValue(), 0xffffffffffffffffL);
    }
}

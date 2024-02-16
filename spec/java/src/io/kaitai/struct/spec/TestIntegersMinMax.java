// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.IntegersMinMax;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestIntegersMinMax extends CommonSpec {

    @Test
    public void testIntegersMinMax() throws Exception {
        IntegersMinMax r = IntegersMinMax.fromFile(SRC_DIR + "integers_min_max.bin");

        assertIntEquals(r.unsignedMin().u1(), 0);
        assertIntEquals(r.unsignedMin().u2le(), 0);
        assertIntEquals(r.unsignedMin().u4le(), 0);
        assertIntEquals(r.unsignedMin().u8le(), 0);
        assertIntEquals(r.unsignedMin().u2be(), 0);
        assertIntEquals(r.unsignedMin().u4be(), 0);
        assertIntEquals(r.unsignedMin().u8be(), 0);
        assertIntEquals(r.unsignedMax().u1(), 255);
        assertIntEquals(r.unsignedMax().u2le(), 65535);
        assertIntEquals(r.unsignedMax().u4le(), 4294967295L);
        assertIntEquals(r.unsignedMax().u8le(), 0xffffffffffffffffL);
        assertIntEquals(r.unsignedMax().u2be(), 65535);
        assertIntEquals(r.unsignedMax().u4be(), 4294967295L);
        assertIntEquals(r.unsignedMax().u8be(), 0xffffffffffffffffL);
        assertIntEquals(r.signedMin().s1(), -128);
        assertIntEquals(r.signedMin().s2le(), -32768);
        assertIntEquals(r.signedMin().s4le(), -2147483648);
        assertIntEquals(r.signedMin().s8le(), -9223372036854775808L);
        assertIntEquals(r.signedMin().s2be(), -32768);
        assertIntEquals(r.signedMin().s4be(), -2147483648);
        assertIntEquals(r.signedMin().s8be(), -9223372036854775808L);
        assertIntEquals(r.signedMax().s1(), 127);
        assertIntEquals(r.signedMax().s2le(), 32767);
        assertIntEquals(r.signedMax().s4le(), 2147483647);
        assertIntEquals(r.signedMax().s8le(), 9223372036854775807L);
        assertIntEquals(r.signedMax().s2be(), 32767);
        assertIntEquals(r.signedMax().s4be(), 2147483647);
        assertIntEquals(r.signedMax().s8be(), 9223372036854775807L);
    }
}

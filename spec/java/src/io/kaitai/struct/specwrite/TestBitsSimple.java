package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.BitsSimple;
import org.testng.annotations.Test;

public class TestBitsSimple extends CommonSpec {
    @Test
    public void testBitsSimple() throws Exception {
        BitsSimple r = new BitsSimple();

        r.setByte1(80);
        r.setByte2(65);
        r.setBitsA(false);
        r.setBitsB(4);
        r.setBitsC(3);
        r.setLargeBits1(300);
        r.setSpacer(5);
        r.setLargeBits2(1329);
        r.setNormalS2((short) -1);
        r.setByte8910(5259587);
        r.setByte11To14(1261262125);
        r.setByte15To19(293220057087L);
        r.setByte20To27(0xffffffffffffffffL);

        assertEqualToFile(r, "fixed_struct.bin");
    }
}

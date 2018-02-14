package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.FloatToI;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestFloatToI extends CommonSpec {
    @Test
    public void testFloatToI() throws Exception {
        FloatToI r = FloatToI.fromFile(SRC_DIR + "floating_points.bin");

        assertEquals(r.singleValue(), 0.5f);
        assertEquals(r.doubleValue(), 0.25);
        
        assertEquals(r.singleI().intValue(), 0);
        assertEquals(r.doubleI().intValue(), 0);
        assertEquals(r.float1I().intValue(), 1);
        assertEquals(r.float2I().intValue(), 1);
        assertEquals(r.float3I().intValue(), 1);
        assertEquals(r.float4I().intValue(), -2);
    }
}

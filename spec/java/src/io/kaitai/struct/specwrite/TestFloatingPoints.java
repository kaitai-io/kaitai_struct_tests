package io.kaitai.struct.specwrite;

import io.kaitai.struct.testwrite.FloatingPoints;
import org.testng.annotations.Test;

public class TestFloatingPoints extends CommonSpec {
    @Test
    public void testFloatingPoints() throws Exception {
        FloatingPoints r = new FloatingPoints(emptyIO());

        r.setSingleValue(0.5f);
        r.setSingleValueBe(0.5f);

        r.setDoubleValue(0.25d);
        r.setDoubleValueBe(0.25d);

        r.setApproximateValue(1.2345f);

        r._write();

        assertEqualToFile(r, "floating_points.bin");
    }
}

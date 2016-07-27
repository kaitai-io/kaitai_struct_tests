package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.FloatingPoints;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestFloatingPoints extends CommonSpec {
    @Test
    public void testFloatingPoints() throws Exception {
        FloatingPoints fp = FloatingPoints.fromFile(SRC_DIR + "floating_points.bin");

        double delta = 1e-6;

        assertEquals(0.5f, fp.singleValue());
        assertEquals(0.5f, fp.singleValueBe());

        assertEquals(0.25d, fp.doubleValue());
        assertEquals(0.25d, fp.doubleValueBe());

        assertEquals(1.2345d, fp.approximateValue(), delta);

        assertEquals(1.5f, fp.singleValuePlusInt(), delta);
        assertEquals(1.0d, fp.singleValuePlusFloat(), delta);
        assertEquals(0.3d, fp.doubleValuePlusFloat(), delta);
    }

}

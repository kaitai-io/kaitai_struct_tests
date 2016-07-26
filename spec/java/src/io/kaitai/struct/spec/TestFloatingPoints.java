package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.FloatingPoints;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestFloatingPoints extends CommonSpec {
    @Test
    public void testFloatingPoints() throws Exception {
        FloatingPoints fp = FloatingPoints.fromFile(SRC_DIR + "floating_points.bin");

        double delta = 1e-6;

        assertEquals(1.2345f, fp.singleValue(), delta);
        assertEquals(1.2345f, fp.singleValueBe(), delta);

        assertEquals(123.456d, fp.doubleValue(), delta);
        assertEquals(123.456d, fp.doubleValueBe(), delta);

        assertEquals(2.2345f, fp.singleValuePlusInt(), delta);
        assertEquals(1.7345d, fp.singleValuePlusFloat(), delta);
        assertEquals(123.506d, fp.doubleValuePlusFloat(), delta);
    }

}

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.*;
import org.testng.annotations.Test;

import java.nio.charset.Charset;
import java.util.ArrayList;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertEqualsNoOrder;

public class TestPositionInSeq extends CommonSpec {
    @Test
    public void testPositionInSeq() throws Exception {
        PositionInSeq r = PositionInSeq.fromFile(SRC_DIR + "position_in_seq.bin");

        ArrayList<Integer> expected = new ArrayList<Integer>(3);
        expected.add(1);
        expected.add(2);
        expected.add(3);

        assertEquals(r.numbers(), expected);
    }

}

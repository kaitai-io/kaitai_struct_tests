// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BytesPadTermZeroSize;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBytesPadTermZeroSize extends CommonSpec {
    @Test
    public void testBytesPadTermZeroSize() throws Exception {
        BytesPadTermZeroSize r = BytesPadTermZeroSize.fromFile(SRC_DIR + "enum_negative.bin");

        assertEquals(r.strPad(), new byte[] {  });
        assertEquals(r.strTerm(), new byte[] {  });
        assertEquals(r.strTermAndPad(), new byte[] {  });
        assertEquals(r.strTermInclude(), new byte[] {  });
    }
}

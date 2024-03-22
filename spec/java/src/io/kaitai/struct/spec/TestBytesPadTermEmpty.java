// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.BytesPadTermEmpty;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestBytesPadTermEmpty extends CommonSpec {
    @Test
    public void testBytesPadTermEmpty() throws Exception {
        BytesPadTermEmpty r = BytesPadTermEmpty.fromFile(SRC_DIR + "str_pad_term_empty.bin");

        assertEquals(r.strPad(), new byte[] {  });
        assertEquals(r.strTerm(), new byte[] {  });
        assertEquals(r.strTermAndPad(), new byte[] {  });
        assertEquals(r.strTermInclude(), new byte[] { 64 });
    }
}

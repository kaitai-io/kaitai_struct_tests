// Autogenerated from KST: please remove this line if doing any edits by hand!

package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.ProcessStructPadTerm;
import org.testng.annotations.Test;
import static org.testng.Assert.*;
public class TestProcessStructPadTerm extends CommonSpec {

    @Test
    public void testProcessStructPadTerm() throws Exception {
        ProcessStructPadTerm r = ProcessStructPadTerm.fromFile(SRC_DIR + "str_pad_term.bin");

        assertEquals(r.strPad().value(), new byte[] { 102, 97, 103, 36 });
        assertEquals(r.strTerm().value(), new byte[] { 102, 97, 103, 39, 115, 122, 122 });
        assertEquals(r.strTermAndPad().value(), new byte[] { 102, 97, 103, 62, 62, 62, 38, 119, 116, 103, 62, 62, 62 });
        assertEquals(r.strTermInclude().value(), new byte[] { 102, 97, 103, 33, 119, 116, 111, 85 });
    }
}
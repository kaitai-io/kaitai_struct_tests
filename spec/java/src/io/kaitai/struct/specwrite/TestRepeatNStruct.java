package io.kaitai.struct.specwrite;

import io.kaitai.struct.KaitaiStream;
import io.kaitai.struct.testwrite.RepeatNStruct;
import org.testng.annotations.Test;

import java.util.ArrayList;

import static org.testng.Assert.assertEquals;

public class TestRepeatNStruct extends CommonSpec {
    @Test
    public void testRepeatNStruct() throws Exception {
        KaitaiStream io = emptyIO();
        RepeatNStruct r = new RepeatNStruct(io);

        ArrayList<RepeatNStruct.Chunk> chunks = new ArrayList<>();
        chunks.add(new RepeatNStruct.Chunk(io) {{
            setOffset(0x10);
            setLen(0x2078);
        }});
        chunks.add(new RepeatNStruct.Chunk(io) {{
            setOffset(0x2088);
            setLen(0xf);
        }});
        r.setQty(chunks.size());
        r.setChunks(chunks);

        r._write();
        assertEqualToFile(r, "repeat_n_struct.bin");
    }
}

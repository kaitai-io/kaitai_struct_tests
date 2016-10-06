package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualIntSize;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualIntSize extends CommonSpec {
    @Test
    public void testSwitchManualIntSize() throws Exception {
        SwitchManualIntSize r = SwitchManualIntSize.fromFile(SRC_DIR + "switch_tlv.bin");

        assertEquals(r.chunks().size(), 4);

        assertEquals(r.chunks().get(0).code(), 0x11);
        SwitchManualIntSize.Chunk.ChunkMeta meta = (SwitchManualIntSize.Chunk.ChunkMeta) r.chunks().get(0).body();
        assertEquals(meta.title(), "Stuff");
        assertEquals(meta.author(), "Me");

        assertEquals(r.chunks().get(1).code(), 0x22);
        assertEquals(
                ((SwitchManualIntSize.Chunk.ChunkDir) r.chunks().get(1).body()).entries().toArray(),
                new String[] { "AAAA", "BBBB", "CCCC" }
        );

        assertEquals(r.chunks().get(2).code(), 0x33);
        assertEquals(r.chunks().get(2).body(), new byte[] { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, (byte) 0x80 });

        assertEquals(r.chunks().get(3).code(), 0xff);
        assertEquals(r.chunks().get(3).body(), new byte[] {});
    }
}

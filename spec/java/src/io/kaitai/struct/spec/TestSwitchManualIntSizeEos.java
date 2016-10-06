package io.kaitai.struct.spec;

import io.kaitai.struct.testformats.SwitchManualIntSizeEos;
import org.testng.annotations.Test;

import static org.testng.Assert.assertEquals;

public class TestSwitchManualIntSizeEos extends CommonSpec {
    @Test
    public void testSwitchManualIntSizeEos() throws Exception {
        SwitchManualIntSizeEos r = SwitchManualIntSizeEos.fromFile(SRC_DIR + "switch_tlv.bin");

        assertEquals(r.chunks().size(), 4);

        assertEquals(r.chunks().get(0).code(), 0x11);
        SwitchManualIntSizeEos.ChunkBody.ChunkMeta meta = (SwitchManualIntSizeEos.ChunkBody.ChunkMeta) r.chunks().get(0).body().body();
        assertEquals(meta.title(), "Stuff");
        assertEquals(meta.author(), "Me");

        assertEquals(r.chunks().get(1).code(), 0x22);
        assertEquals(
                ((SwitchManualIntSizeEos.ChunkBody.ChunkDir) r.chunks().get(1).body().body()).entries().toArray(),
                new String[] { "AAAA", "BBBB", "CCCC" }
        );

        assertEquals(r.chunks().get(2).code(), 0x33);
        assertEquals(r.chunks().get(2).body().body(), new byte[] { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, (byte) 0x80 });

        assertEquals(r.chunks().get(3).code(), 0xff);
        assertEquals(r.chunks().get(3).body().body(), new byte[] {});
    }
}

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualIntSizeElse : CommonSpec
    {
        [Test]
        public void TestSwitchManualIntSizeElse()
        {
            var r = SwitchManualIntSizeElse.FromFile(SourceFile("switch_tlv.bin"));
            Assert.AreEqual(r.Chunks.Count, 4);

            Assert.AreEqual(r.Chunks[0].Code, 0x11);
            var meta = (SwitchManualIntSizeElse.Chunk.ChunkMeta)r.Chunks[0].Body;
            Assert.AreEqual(meta.Title, "Stuff");
            Assert.AreEqual(meta.Author, "Me");

            Assert.AreEqual(r.Chunks[1].Code, 0x22);
            Assert.AreEqual(((SwitchManualIntSizeElse.Chunk.ChunkDir) r.Chunks[1].Body).Entries, new string[] { "AAAA", "BBBB", "CCCC" });

            Assert.AreEqual(r.Chunks[2].Code, 0x33);
            Assert.AreEqual(((SwitchManualIntSizeElse.Chunk.Dummy) r.Chunks[2].Body).Rest, new byte[] { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80 });

            Assert.AreEqual(r.Chunks[3].Code, 0xff);
            Assert.AreEqual(((SwitchManualIntSizeElse.Chunk.Dummy) r.Chunks[3].Body).Rest, new byte[] {});
        }
    }
}

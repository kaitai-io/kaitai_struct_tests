using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualIntSize : CommonSpec
    {
        [Test]
        public void TestSwitchManualIntSize()
        {
            var r = SwitchManualIntSize.FromFile(SourceFile("switch_tlv.bin"));
            Assert.AreEqual(r.Chunks.Count, 4);
    
            Assert.AreEqual(r.Chunks[0].Code, 0x11);
            var meta = (SwitchManualIntSize.Chunk.ChunkMeta)r.Chunks[0].Body;
            Assert.AreEqual(meta.Title, "Stuff");
            Assert.AreEqual(meta.Author, "Me");
    
            Assert.AreEqual(r.Chunks[1].Code, 0x22);
            Assert.AreEqual(((SwitchManualIntSize.Chunk.ChunkDir) r.Chunks[1].Body).Entries, new string[] { "AAAA", "BBBB", "CCCC" });
    
            Assert.AreEqual(r.Chunks[2].Code, 0x33);
            Assert.AreEqual(r.Chunks[2].Body, new byte[] { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80 });
    
            Assert.AreEqual(r.Chunks[3].Code, 0xff);
            Assert.AreEqual(r.Chunks[3].Body, new byte[] {});
        }
    }
}

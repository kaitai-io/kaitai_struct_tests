using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecSwitchManualIntSizeEos : CommonSpec
    {
        [Test]
        public async Task TestSwitchManualIntSizeEos()
        {
            var r = SwitchManualIntSizeEos.FromFile(SourceFile("switch_tlv.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Chunks.Count, 4);

            Assert.AreEqual(r.Chunks[0].Code, 0x11);
            var meta = (SwitchManualIntSizeEos.ChunkBody.ChunkMeta)r.Chunks[0].Body.Body;
            Assert.AreEqual(meta.Title, "Stuff");
            Assert.AreEqual(meta.Author, "Me");

            Assert.AreEqual(r.Chunks[1].Code, 0x22);
            Assert.AreEqual(((SwitchManualIntSizeEos.ChunkBody.ChunkDir) r.Chunks[1].Body.Body).Entries, new string[] { "AAAA", "BBBB", "CCCC" });

            Assert.AreEqual(r.Chunks[2].Code, 0x33);
            Assert.AreEqual(r.Chunks[2].Body.Body, new byte[] { 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80 });

            Assert.AreEqual(r.Chunks[3].Code, 0xff);
            Assert.AreEqual(r.Chunks[3].Body.Body, new byte[] {});
        }
    }
}

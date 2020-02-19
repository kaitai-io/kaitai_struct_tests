using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprIoEof : CommonSpec
    {
        [Test]
        public async Task TestExprIoEof()
        {
            var r = ExprIoEof.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();
            Assert.AreEqual(r.Substream1.One, 1262698832);
            Assert.AreEqual(r.Substream1.Two, null);
    
            Assert.AreEqual(r.Substream2.One, 4294914349);
            Assert.AreEqual(r.Substream2.Two, 1262698832);
        }
    }
}

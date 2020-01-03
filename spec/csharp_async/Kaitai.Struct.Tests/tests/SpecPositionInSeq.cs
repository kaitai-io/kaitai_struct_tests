using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecPositionInSeq : CommonSpec
    {
        [Test]
        public async Task TestPositionInSeq()
        {
            PositionInSeq r = PositionInSeq.FromFile(SourceFile("position_in_seq.bin"));
            await r.ReadAsync();

            CollectionAssert.AreEqual(r.Numbers, new[] {1, 2, 3});
        }
    }
}
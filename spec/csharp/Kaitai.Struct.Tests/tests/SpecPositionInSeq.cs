using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecPositionInSeq : CommonSpec
    {
        [Test]
        public void TestPositionInSeq()
        {
            PositionInSeq r = PositionInSeq.FromFile(SourceFile("position_in_seq.bin"));

            CollectionAssert.AreEqual(r.Numbers, new[] {1, 2, 3});
        }
    }
}
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecPositionAbs : CommonSpec
    {
        [Test]
        public void TestPositionAbs()
        {
            PositionAbs r = PositionAbs.FromFile(SourceFile("position_abs.bin"));

            Assert.AreEqual(r.IndexOffset, 0x20);
            Assert.AreEqual(r.Index.Entry, "foo");
        }
    }
}
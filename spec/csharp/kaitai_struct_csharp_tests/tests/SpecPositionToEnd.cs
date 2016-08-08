using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecPositionToEnd : CommonSpec
    {
        [Test]
        public void TestPositionToEnd()
        {
            PositionToEnd r = PositionToEnd.FromFile(SourceFile("position_to_end.bin"));

            Assert.AreEqual(r.Index.Foo, 0x42);
            Assert.AreEqual(r.Index.Bar, 0x1234);
        }
    }
}
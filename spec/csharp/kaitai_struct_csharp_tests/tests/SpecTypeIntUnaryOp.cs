using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTypeIntUnaryOp : CommonSpec
    {
        [Test]
        public void TestTypeIntUnaryOp()
        {
            var r = TypeIntUnaryOp.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.ValueS2, 0x4150);
            Assert.AreEqual(r.ValueS8, 0x4150ffff312d4b43);
            Assert.AreEqual(r.UnaryS2, -0x4150);
            Assert.AreEqual(r.UnaryS8, -0x4150ffff312d4b43);
        }
    }
}

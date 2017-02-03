using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecBitsEnum : CommonSpec
    {
        [Test]
        public void TestBitsEnum()
        {
            var r = BitsEnum.FromFile(SourceFile("fixed_struct.bin"));
            // 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
            Assert.AreEqual(r.One, BitsEnum.Animal.Platypus);
            Assert.AreEqual(r.Two, BitsEnum.Animal.Horse);
            Assert.AreEqual(r.Three, BitsEnum.Animal.Cat);
        }
    }
}

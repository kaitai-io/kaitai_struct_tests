using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianMod : CommonSpec
    {
        [Test]
        public void TestDefaultEndianMod()
        {
            var r = DefaultEndianMod.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Main.One, 0x4b434150);
            Assert.AreEqual(r.Main.Nest.Two, -52947);
            Assert.AreEqual(r.Main.NestBe.Two, 0x5041434b);
        }
    }
}

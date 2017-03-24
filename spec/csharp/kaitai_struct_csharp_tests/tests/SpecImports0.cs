using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImports0 : CommonSpec
    {
        [Test]
        public void TestImports0()
        {
            var r = Imports0.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Two, 0x50);
            Assert.AreEqual(r.Hw.One, 0x41);
            Assert.AreEqual(r.HwOne, 0x41);
        }
    }
}

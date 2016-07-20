using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultBigEndian : CommonSpec
    {
        [Test]
        public void TestDefaultBigEndian()
        {
            var r = DefaultBigEndian.FromFile(SourceFile("enum_0.bin"));

            Assert.AreEqual(r.One, 0x7000000);
        }
    }
}

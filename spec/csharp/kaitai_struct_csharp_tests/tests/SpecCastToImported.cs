using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecCastToImported : CommonSpec
    {
        [Test]
        public void TestCastToImported()
        {
            var r = CastToImported.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.One.One, 0x50);
            Assert.AreEqual(r.OneCasted.One, 0x50);
        }
    }
}

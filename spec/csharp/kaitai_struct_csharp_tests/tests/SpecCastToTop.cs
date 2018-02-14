using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecCastToTop : CommonSpec
    {
        [Test]
        public void TestCastToTop()
        {
            var r = CastToTop.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Code, 0x50);
            Assert.AreEqual(r.Header.Code, 0x41);
            Assert.AreEqual(r.HeaderCasted.Code, 0x41);
        }
    }
}

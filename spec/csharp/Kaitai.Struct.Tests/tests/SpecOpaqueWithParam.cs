using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOpaqueWithParam : CommonSpec
    {
        [Test]
        public void TestOpaqueWithParam()
        {
            var r = OpaqueWithParam.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(r.One.Buf, "foo|b");
            Assert.AreEqual(r.One.Trailer, 0x61);
        }
    }
}

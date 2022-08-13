using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOpaqueExternalType : CommonSpec
    {
        [Test]
        public void TestOpaqueExternalType()
        {
            var r = OpaqueExternalType.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.Hw.One, 102);
        }
    }
}

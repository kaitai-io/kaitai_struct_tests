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

            Assert.AreEqual(102, r.Hw.One);
        }
    }
}

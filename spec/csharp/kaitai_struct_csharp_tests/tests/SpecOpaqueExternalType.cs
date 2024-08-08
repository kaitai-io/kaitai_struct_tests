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
            AreEqual(r.One.S1, "foo");
            AreEqual(r.One.S2, "bar");
            AreEqual(r.One.S3, "|baz@");
        }
    }
}

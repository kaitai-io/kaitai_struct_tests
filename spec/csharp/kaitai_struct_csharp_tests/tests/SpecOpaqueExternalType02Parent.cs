using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOpaqueExternalType02Parent : CommonSpec
    {
        [Test]
        public void TestOpaqueExternalType02Parent()
        {
            var r = OpaqueExternalType02Parent.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.Parent.Child.S1, "foo");
            Assert.AreEqual(r.Parent.Child.S2, "bar");
            Assert.AreEqual(r.Parent.Child.S3.S3, "|baz@");
        }
    }
}

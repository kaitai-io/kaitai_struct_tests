using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTypeTernaryOpaque : CommonSpec
    {
        [Test]
        public void TestTypeTernaryOpaque()
        {
            var r = TypeTernaryOpaque.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(r.Dif.S1, "foo");
            Assert.AreEqual(r.Dif.S2, "bar");
            Assert.AreEqual(r.Dif.S3, "|baz@");
        }
    }
}

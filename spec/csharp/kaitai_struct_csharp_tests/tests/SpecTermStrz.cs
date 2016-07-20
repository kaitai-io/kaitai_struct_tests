using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTermStrz : CommonSpec
    {
        [Test]
        public void TestTermStrz()
        {
            TermStrz r = TermStrz.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.S1, "foo");
            Assert.AreEqual(r.S2, "bar");
            Assert.AreEqual(r.S3, "|baz@");
        }
    }
}
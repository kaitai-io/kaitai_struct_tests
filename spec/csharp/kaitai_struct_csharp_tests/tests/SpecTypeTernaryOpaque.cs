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

            Assert.AreEqual(102, r.Dif.One);
        }
    }
}

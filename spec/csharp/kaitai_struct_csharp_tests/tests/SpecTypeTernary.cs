using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTypeTernary : CommonSpec
    {
        [Test]
        public void TestTypeTernary()
        {
            var r = TypeTernary.FromFile(SourceFile("term_strz.bin"));
            Assert.AreEqual(r.Dif.Value, 0x65);
        }
    }
}

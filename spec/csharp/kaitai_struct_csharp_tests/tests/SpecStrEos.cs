using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecStrEos : CommonSpec
    {
        [Test]
        public void TestStrEos()
        {
            StrEos r = StrEos.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.Str, "foo|bar|baz@");
        }
    }
}
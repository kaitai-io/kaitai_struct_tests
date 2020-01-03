using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecTypeTernaryOpaque : CommonSpec
    {
        [Test]
        public async Task TestTypeTernaryOpaque()
        {
            var r = TypeTernaryOpaque.FromFile(SourceFile("term_strz.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Dif.S1, "foo");
            Assert.AreEqual(r.Dif.S2, "bar");
            Assert.AreEqual(r.Dif.S3, "|baz@");
        }
    }
}

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOpaqueExternalType : CommonSpec
    {
        [Test]
        public async Task TestOpaqueExternalType()
        {
            var r = OpaqueExternalType.FromFile(SourceFile("term_strz.bin"));
            await r.ReadAsync();
            Assert.AreEqual(r.One.S1, "foo");
            Assert.AreEqual(r.One.S2, "bar");
            Assert.AreEqual(r.One.S3, "|baz@");
        }
    }
}

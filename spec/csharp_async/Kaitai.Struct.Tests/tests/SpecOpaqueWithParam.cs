using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOpaqueWithParam : CommonSpec
    {
        [Test]
        public async Task TestOpaqueWithParam()
        {
            var r = OpaqueWithParam.FromFile(SourceFile("term_strz.bin"));
            await r.ReadAsync();
            Assert.AreEqual(r.One.Buf, "foo|b");
            Assert.AreEqual(r.One.Trailer, 0x61);
        }
    }
}

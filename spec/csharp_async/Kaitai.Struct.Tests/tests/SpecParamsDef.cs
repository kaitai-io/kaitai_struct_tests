using System.Threading.Tasks;
using Kaitai.Async;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecParamsDef : CommonSpec
    {
        [Test]
        public async Task TestParamsDef()
        {
            var io = new KaitaiAsyncStream(SourceFile("term_strz.bin"));
            var r = new ParamsDef(5, true, io);
            await r.ReadAsync();

            Assert.AreEqual(r.Buf, "foo|b");
            Assert.AreEqual(r.Trailer, 0x61);
        }
    }
}

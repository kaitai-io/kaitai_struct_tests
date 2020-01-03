// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecZlibWithHeader78 : CommonSpec
    {
        [Test]
        public async Task TestZlibWithHeader78()
        {
            var r = ZlibWithHeader78.FromFile(SourceFile("zlib_with_header_78.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Data, new byte[] { 97, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110, 32, 102, 111, 120, 32, 106, 117, 109, 112, 115, 32, 111, 118, 101, 114 });
        }
    }
}

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImportsAbs : CommonSpec
    {
        [Test]
        public async Task TestImportsAbs()
        {
            var r = ImportsAbs.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Len.Value, 80);
            Assert.AreEqual(r.Body.Length, 80);
        }
    }
}

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImportsCircularA : CommonSpec
    {
        [Test]
        public async Task TestImportsCircularA()
        {
            var r = ImportsCircularA.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();
            Assert.AreEqual(r.Code, 0x50);
            Assert.AreEqual(r.Two.Initial, 0x41);
            Assert.AreEqual(r.Two.BackRef.Code, 0x43);
            Assert.AreEqual(r.Two.BackRef.Two.Initial, 0x4b);
            Assert.AreEqual(r.Two.BackRef.Two.BackRef, null);
        }
    }
}

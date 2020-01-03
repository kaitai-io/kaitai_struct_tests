// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecImports0 : CommonSpec
    {
        [Test]
        public async Task TestImports0()
        {
            var r = Imports0.FromFile(SourceFile("fixed_struct.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Two, 80);
            Assert.AreEqual(r.Hw.One, 65);
            Assert.AreEqual(r.HwOne, 65);
        }
    }
}

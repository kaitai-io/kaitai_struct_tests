// Autogenerated from KST: please remove this line if doing any edits by hand!

using System.Threading.Tasks;
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprIsLe : CommonSpec
    {
        [Test]
        public async Task TestDefaultEndianExprIsLe()
        {
            var r = DefaultEndianExprIsLe.FromFile(SourceFile("endian_expr.bin"));
            await r.ReadAsync();

            Assert.AreEqual(r.Docs[0].Indicator, new byte[] { 73, 73 });
            Assert.AreEqual(r.Docs[0].Main.SomeInt, 66);
            Assert.AreEqual(r.Docs[0].Main.SomeIntBe, 66);
            Assert.AreEqual(r.Docs[0].Main.SomeIntLe, 66);
            Assert.AreEqual(r.Docs[1].Indicator, new byte[] { 77, 77 });
            Assert.AreEqual(r.Docs[1].Main.SomeInt, 66);
            Assert.AreEqual(r.Docs[1].Main.SomeIntBe, 66);
            Assert.AreEqual(r.Docs[1].Main.SomeIntLe, 66);
            Assert.AreEqual(r.Docs[2].Indicator, new byte[] { 88, 88 });
            Assert.AreEqual(r.Docs[2].Main.SomeInt, 66);
            Assert.AreEqual(r.Docs[2].Main.SomeIntBe, 66);
            Assert.AreEqual(r.Docs[2].Main.SomeIntLe, 66);
        }
    }
}

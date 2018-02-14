using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprIsLe : CommonSpec
    {
        [Test]
        public void TestDefaultEndianExprIsLe()
        {
            var r = DefaultEndianExprIsLe.FromFile(SourceFile("endian_expr.bin"));
            Assert.AreEqual(r.Docs[0].Indicator, new byte[] { 0x49, 0x49 });
            Assert.AreEqual(r.Docs[0].Main.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[0].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[0].Main.SomeIntLe, 0x42);
    
            Assert.AreEqual(r.Docs[1].Indicator, new byte[] { 0x4d, 0x4d });
            Assert.AreEqual(r.Docs[1].Main.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[1].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[1].Main.SomeIntLe, 0x42);
    
            Assert.AreEqual(r.Docs[2].Indicator, new byte[] { 0x58, 0x58 });
            Assert.AreEqual(r.Docs[2].Main.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[2].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[2].Main.SomeIntLe, 0x42);
        }
    }
}

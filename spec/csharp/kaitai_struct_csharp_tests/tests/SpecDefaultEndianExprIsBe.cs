using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprIsBe : CommonSpec
    {
        [Test]
        public void TestDefaultEndianExprIsBe()
        {
            var r = DefaultEndianExprIsBe.FromFile(SourceFile("endian_expr.bin"));
            // LE
            Assert.AreEqual(r.Docs[0].Indicator, new byte[] { 0x49, 0x49 });
            Assert.AreEqual(r.Docs[0].Main.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[0].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[0].Main.SomeIntLe, 0x42);
    
            Assert.AreEqual(r.Docs[0].Main.InstInt, 0x42);
            Assert.AreEqual(r.Docs[0].Main.InstSub.Foo, 0x42);
    
            // BE
            Assert.AreEqual(r.Docs[1].Indicator, new byte[] { 0x4d, 0x4d });
            Assert.AreEqual(r.Docs[1].Main.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[1].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[1].Main.SomeIntLe, 0x42);
    
            Assert.AreEqual(r.Docs[1].Main.InstInt, 0x42000000);
            Assert.AreEqual(r.Docs[1].Main.InstSub.Foo, 0x42000000);
    
            // Weird => LE
            Assert.AreEqual(r.Docs[2].Indicator, new byte[] { 0x58, 0x58 });
            Assert.AreEqual(r.Docs[2].Main.SomeInt, 0x42000000);
            Assert.AreEqual(r.Docs[2].Main.SomeIntBe, 0x42);
            Assert.AreEqual(r.Docs[2].Main.SomeIntLe, 0x42);
    
            Assert.AreEqual(r.Docs[2].Main.InstInt, 0x42);
            Assert.AreEqual(r.Docs[2].Main.InstSub.Foo, 0x42);
        }
    }
}

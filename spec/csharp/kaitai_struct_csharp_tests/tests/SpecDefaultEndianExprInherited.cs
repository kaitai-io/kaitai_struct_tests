using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDefaultEndianExprInherited : CommonSpec
    {
        [Test]
        public void TestDefaultEndianExprInherited()
        {
            var r = DefaultEndianExprInherited.FromFile(SourceFile("endian_expr.bin"));
            Assert.AreEqual(r.Docs[0].Indicator, new byte[] { 0x49, 0x49 });
            Assert.AreEqual(r.Docs[0].Main.Insides.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[0].Main.Insides.More.SomeInt1, 0x4200);
            Assert.AreEqual(r.Docs[0].Main.Insides.More.SomeInt2, 0x42);
            Assert.AreEqual(r.Docs[0].Main.Insides.More.SomeInst, 0x42);
    
            Assert.AreEqual(r.Docs[1].Indicator, new byte[] { 0x4d, 0x4d });
            Assert.AreEqual(r.Docs[1].Main.Insides.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[1].Main.Insides.More.SomeInt1, 0x42);
            Assert.AreEqual(r.Docs[1].Main.Insides.More.SomeInt2, 0x4200);
            Assert.AreEqual(r.Docs[1].Main.Insides.More.SomeInst, 0x42000000);
    
            Assert.AreEqual(r.Docs[2].Indicator, new byte[] { 0x58, 0x58 });
            Assert.AreEqual(r.Docs[2].Main.Insides.SomeInt, 0x42);
            Assert.AreEqual(r.Docs[2].Main.Insides.More.SomeInt1, 0x42);
            Assert.AreEqual(r.Docs[2].Main.Insides.More.SomeInt2, 0x4200);
            Assert.AreEqual(r.Docs[2].Main.Insides.More.SomeInst, 0x42000000);
        }
    }
}

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExpr1 : CommonSpec
    {
        [Test]
        public void TestExpr1()
        {
            Expr1 r = Expr1.FromFile(SourceFile("str_encodings.bin"));

            Assert.AreEqual(r.LenOf1, 10);
            Assert.AreEqual(r.LenOf1Mod, 8);
            Assert.AreEqual(r.Str1, "Some ASC");
            Assert.AreEqual(r.Str1Len, 8);
        }
    }
}
using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExpr2 : CommonSpec
    {
        [Test]
        public void TestExpr2()
        {
            Expr2 r = Expr2.FromFile(SourceFile("str_encodings.bin"));

            Assert.AreEqual(r.Str1.LenOrig, 10);
            Assert.AreEqual(r.Str1.LenMod, 7);
            Assert.AreEqual(r.Str1.Str, "Some AS");

            Assert.AreEqual(r.Str1Len, 7);
            Assert.AreEqual(r.Str1LenMod, 7);
            Assert.AreEqual(r.Str1Byte1, 0x49);
            Assert.AreEqual(r.Str1Avg, 0x49);
            Assert.AreEqual(r.Str1Char5, "e");

            Assert.AreEqual(r.Str1Tuple5.Byte0, 0x65);
            Assert.AreEqual(r.Str1Tuple5.Byte1, 0x20);
            Assert.AreEqual(r.Str1Tuple5.Byte2, 0x41);
            Assert.AreEqual(r.Str1Tuple5.Avg, 0x30);

            Assert.AreEqual(r.Str2Tuple5.Byte0, 0x65);
            Assert.AreEqual(r.Str2Tuple5.Byte1, 0x20);
            Assert.AreEqual(r.Str2Tuple5.Byte2, 0x41);
            Assert.AreEqual(r.Str2Tuple5.Avg, 0x30);
        }
    }
}
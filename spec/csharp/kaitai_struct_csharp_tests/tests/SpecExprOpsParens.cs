// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprOpsParens : CommonSpec
    {
        [Test]
        public void TestExprOpsParens()
        {
            var r = ExprOpsParens.FromFile(SourceFile("enum_negative.bin"));

            Assert.AreEqual("29", r.ISumToStr);
            Assert.AreEqual(9, r.FSumToInt);
            Assert.AreEqual(10, r.StrConcatLen);
            Assert.AreEqual("9876543210", r.StrConcatRev);
            Assert.AreEqual("23456", r.StrConcatSubstr2To7);
            Assert.AreEqual(123456789, r.StrConcatToI);
            Assert.AreEqual(0, r.BoolEq);
            Assert.AreEqual(0, r.BoolAnd);
            Assert.AreEqual(1, r.BoolOr);
        }
    }
}

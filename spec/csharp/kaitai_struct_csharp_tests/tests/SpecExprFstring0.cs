// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecExprFstring0 : CommonSpec
    {
        [Test]
        public void TestExprFstring0()
        {
            var r = ExprFstring0.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.SeqStr, "foo|b");
            Assert.AreEqual(r.SeqInt, 97);
            Assert.AreEqual(r.Empty, "");
            Assert.AreEqual(r.Literal, "abc");
            Assert.AreEqual(r.LiteralWithEscapes, "abc\n\tt");
            Assert.AreEqual(r.HeadAndIntLiteral, "abc=123");
            Assert.AreEqual(r.HeadAndStrLiteral, "abc=foo");
            Assert.AreEqual(r.HeadAndInt, "abc=97");
            Assert.AreEqual(r.HeadAndStr, "abc=foo|b");
        }
    }
}

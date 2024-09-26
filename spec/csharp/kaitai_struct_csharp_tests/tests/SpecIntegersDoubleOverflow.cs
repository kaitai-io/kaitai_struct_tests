// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIntegersDoubleOverflow : CommonSpec
    {
        [Test]
        public void TestIntegersDoubleOverflow()
        {
            var r = IntegersDoubleOverflow.FromFile(SourceFile("integers_double_overflow.bin"));

            Assert.AreEqual(-9007199254740991, r.SignedSafeMinBe);
            Assert.AreEqual(-9007199254740991, r.SignedSafeMinLe);
            Assert.AreEqual(9007199254740991, r.SignedSafeMaxBe);
            Assert.AreEqual(9007199254740991, r.SignedSafeMaxLe);
            Assert.AreEqual("-9007199254740993", r.SignedUnsafeNegBe.ToString());
            Assert.AreEqual("-9007199254740993", r.SignedUnsafeNegLe.ToString());
            Assert.AreEqual("9007199254740993", r.SignedUnsafePosBe.ToString());
            Assert.AreEqual("9007199254740993", r.SignedUnsafePosBe.ToString());
        }
    }
}

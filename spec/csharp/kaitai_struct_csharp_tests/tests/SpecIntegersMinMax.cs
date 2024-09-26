// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecIntegersMinMax : CommonSpec
    {
        [Test]
        public void TestIntegersMinMax()
        {
            var r = IntegersMinMax.FromFile(SourceFile("integers_min_max.bin"));

            Assert.AreEqual(0, r.UnsignedMin.U1);
            Assert.AreEqual(0, r.UnsignedMin.U2le);
            Assert.AreEqual(0, r.UnsignedMin.U4le);
            Assert.AreEqual(0, r.UnsignedMin.U8le);
            Assert.AreEqual(0, r.UnsignedMin.U2be);
            Assert.AreEqual(0, r.UnsignedMin.U4be);
            Assert.AreEqual(0, r.UnsignedMin.U8be);
            Assert.AreEqual(255, r.UnsignedMax.U1);
            Assert.AreEqual(65535, r.UnsignedMax.U2le);
            Assert.AreEqual(4294967295, r.UnsignedMax.U4le);
            Assert.AreEqual(18446744073709551615, r.UnsignedMax.U8le);
            Assert.AreEqual(65535, r.UnsignedMax.U2be);
            Assert.AreEqual(4294967295, r.UnsignedMax.U4be);
            Assert.AreEqual(18446744073709551615, r.UnsignedMax.U8be);
            Assert.AreEqual(-128, r.SignedMin.S1);
            Assert.AreEqual(-32768, r.SignedMin.S2le);
            Assert.AreEqual(-2147483648, r.SignedMin.S4le);
            Assert.AreEqual(-9223372036854775808, r.SignedMin.S8le);
            Assert.AreEqual(-32768, r.SignedMin.S2be);
            Assert.AreEqual(-2147483648, r.SignedMin.S4be);
            Assert.AreEqual(-9223372036854775808, r.SignedMin.S8be);
            Assert.AreEqual(127, r.SignedMax.S1);
            Assert.AreEqual(32767, r.SignedMax.S2le);
            Assert.AreEqual(2147483647, r.SignedMax.S4le);
            Assert.AreEqual(9223372036854775807, r.SignedMax.S8le);
            Assert.AreEqual(32767, r.SignedMax.S2be);
            Assert.AreEqual(2147483647, r.SignedMax.S4be);
            Assert.AreEqual(9223372036854775807, r.SignedMax.S8be);
        }
    }
}

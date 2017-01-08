using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecBitsSimple : CommonSpec
    {
        [Test]
        public void TestBitsSimple()
        {
            var r = BitsSimple.FromFile(SourceFile("fixed_struct.bin"));
            // 50 41
            Assert.AreEqual(r.Byte1, 0x50);
            Assert.AreEqual(r.Byte2, 0x41);

            // 43 (1 + 3 + 4) = 0|100|0011 = 0|4|3
            Assert.AreEqual(r.BitsA, false);
            Assert.AreEqual(r.BitsB, 4);
            Assert.AreEqual(r.BitsC, 3);

            // 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001 = 300|5|1329
            Assert.AreEqual(r.LargeBits1, 300);
            Assert.AreEqual(r.Spacer, 5);
            Assert.AreEqual(r.LargeBits2, 1329);

            // FF FF
            Assert.AreEqual(r.NormalS2, -1);

            // 50 41 43
            Assert.AreEqual(r.Byte8910, 0x504143);
        }
    }
}

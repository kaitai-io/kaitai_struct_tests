using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecBitsByteAligned : CommonSpec
    {
        [Test]
        public void TestBitsByteAligned()
        {
            var r = BitsByteAligned.FromFile(SourceFile("fixed_struct.bin"));
            // 50 (6 + 2) = 010100|00
            Assert.AreEqual(r.One, 20);
            // 41
            Assert.AreEqual(r.Byte1, 0x41);
            // 43 (3 + 1 + 4) = 010|0|0011
            Assert.AreEqual(r.Two, 2);
            Assert.AreEqual(r.Three, false);
            // 4B
            Assert.AreEqual(r.Byte2, 0x4b);
            // 2D 31 (14 + 2) = 00101101 001100|01
            Assert.AreEqual(r.Four, 2892);
            // FF
            Assert.AreEqual(r.Byte3, new byte[] { 0xff });
            // FF
            Assert.AreEqual(r.FullByte, 0xff);
            // 50
            Assert.AreEqual(r.Byte4, 0x50);
        }
    }
}

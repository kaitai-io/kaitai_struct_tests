// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecBitsSignedShiftB32Le : CommonSpec
    {
        [Test]
        public void TestBitsSignedShiftB32Le()
        {
            var r = BitsSignedShiftB32Le.FromFile(SourceFile("bits_signed_shift_b32_le.bin"));

            Assert.AreEqual(0, r.A);
            Assert.AreEqual(255, r.B);
        }
    }
}

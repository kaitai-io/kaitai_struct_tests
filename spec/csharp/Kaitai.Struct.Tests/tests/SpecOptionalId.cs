using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecOptionalId : CommonSpec
    {
        [Test]
        public void TestOptionalId()
        {
            var r = OptionalId.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.Unnamed_0, 0x50);
            Assert.AreEqual(r.Unnamed_1, 0x41);
            Assert.AreEqual(r.Unnamed_2, new byte[] { 0x43, 0x4b, 0x2d, 0x31, 0xff });
        }
    }
}

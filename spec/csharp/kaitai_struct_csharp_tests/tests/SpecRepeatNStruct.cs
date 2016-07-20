using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatNStruct : CommonSpec
    {
        [Test]
        public void TestRepeatNStruct()
        {
            RepeatNStruct r = RepeatNStruct.FromFile(SourceFile("repeat_n_struct.bin"));

            Assert.AreEqual(r.Qty, 2);
            Assert.AreEqual(r.Chunks[0].Offset, 0x10);
            Assert.AreEqual(r.Chunks[0].Len, 0x2078);
            Assert.AreEqual(r.Chunks[1].Offset, 0x2088);
            Assert.AreEqual(r.Chunks[1].Len, 0xf);
        }
    }
}
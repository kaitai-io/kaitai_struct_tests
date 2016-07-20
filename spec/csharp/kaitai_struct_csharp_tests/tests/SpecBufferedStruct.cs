using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecBufferedStruct : CommonSpec
    {
        [Test]
        public void TestBufferedStruct()
        {
            BufferedStruct r = BufferedStruct.FromFile(SourceFile("buffered_struct.bin"));

            Assert.AreEqual(r.Len1, 0x10);
            
            CollectionAssert.AreEqual(r.M_RawBlock1, new byte[]
            {
                0x42, 0, 0, 0,
                0x43, 0, 0, 0,
                0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF,
            });
            Assert.AreEqual(r.Block1.Number1, 0x42);
            Assert.AreEqual(r.Block1.Number2, 0x43);

            Assert.AreEqual(r.Len2, 0x8);

            CollectionAssert.AreEqual(r.M_RawBlock2, new byte[]
            {
                0x44, 0, 0, 0,
                0x45, 0, 0, 0,
            });
            Assert.AreEqual(r.Block2.Number1, 0x44);
            Assert.AreEqual(r.Block2.Number2, 0x45);

            Assert.AreEqual(r.Finisher, 0xee);
        }
    }
}
using NUnit.Framework;

namespace Kaitai
{
    public class CustomFxNoArgs : CustomDecoder
    {
        public CustomFxNoArgs()
        {
        }

        public byte[] Decode(byte[] src)
        {
            byte[] dest = new byte[src.Length + 2];
            src.CopyTo(dest, 1);
            dest[0] = 0x5f;
            dest[dest.Length - 1] = 0x5f;
            return dest;
        }
    }

    [TestFixture]
    public class SpecProcessCustomNoArgs : CommonSpec
    {
        [Test]
        public void TestProcessCustomNoArgs()
        {
            var r = ProcessCustomNoArgs.FromFile(SourceFile("process_rotate.bin"));


            Assert.AreEqual(r.Buf, new byte[] { 95, 9, 172, 141, 141, 237, 95 });
        }
    }
}

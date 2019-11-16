using NUnit.Framework;

namespace Nested
{
    namespace Deeply
    {
        public class CustomFx : Kaitai.CustomDecoder
        {
            public CustomFx(int key)
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
    }
}

namespace Kaitai
{
    public class MyCustomFx : CustomDecoder
    {
        private int key;
        
        public MyCustomFx(int k, bool flag, byte[] someBytes)
        {
            key = flag ? k : -k;
        }

        public byte[] Decode(byte[] src)
        {
            int len = src.Length;
            byte[] dest = new byte[len];
            for (int i = 0; i < len; i++)
            {
                dest[i] = (byte) (src[i] + key);
            }
            return dest;
        }
    }

    [TestFixture]
    public class SpecProcessCustom : CommonSpec
    {
        [Test]
        public void TestProcessCustom()
        {
            var r = ProcessCustom.FromFile(SourceFile("process_rotate.bin"));
            Assert.AreEqual(new byte[] { 0x10, 0xb3, 0x94, 0x94, 0xf4 }, r.Buf1);
            Assert.AreEqual(new byte[] { 0x5f, 0xba, 0x7b, 0x93, 0x63, 0x23, 0x5f }, r.Buf2);
            Assert.AreEqual(new byte[] { 0x29, 0x33, 0xb1, 0x38, 0xb1 }, r.Buf3);
        }
    }
}
